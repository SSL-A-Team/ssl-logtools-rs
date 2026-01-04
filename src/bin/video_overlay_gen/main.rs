use anyhow;
use chrono::{DateTime, Utc};
use clap::Parser;
use indicatif::ProgressBar;
use skia_safe::EncodedImageFormat;
use ssl_logtools_rs::{LogMessage, MessageBody, get_all_referee_messages};
use std::time::Duration;
use std::{io, process::Command};
use std::fs::File;
use std::io::BufReader;
use which::which;

mod colors;
mod templates;

#[derive(Parser)]
#[command(version)]
/// Render SSL referee overlay from game logs
struct Args {
    log_path: String,
    #[arg(default_value_t = ("overlay.mov").to_string())]
    output_path: String,
    #[arg(short = 'f', long, default_value_t = 23.98)]
    frame_rate: f64,
    #[arg(short = 'k', long)]
    /// Skip running ffmpeg to combine frames into a video file
    skip_ffmpeg: bool,
    #[arg(short = 'o', long)]
    /// Directory to output frames to. If not set, frames will be written to a temporary directory.
    frame_output_dir: Option<String>,
    #[arg(short, long)]
    /// Start time in seconds from the beginning of the log
    start_time: Option<f64>,
    #[arg(short, long)]
    /// End time in seconds from the beginning of the log
    end_time: Option<f64>,
    #[arg(short, long)]
    /// Show full ffmpeg output
    verbose_ffmpeg_output: bool,
    #[arg(short, long, default_value_t = ("default").to_string())]
    /// Name of builtin template or path to template file
    template: String,
    #[arg(short, long)]
    /// Path to colors definition JSON file
    colors: Option<String>,
}

fn find_ref_msg_by_time(
    ref_msgs: &Vec<LogMessage>,
    timestamp: DateTime<Utc>,
    start_index: usize,
) -> usize {
    assert!(!ref_msgs.is_empty());
    for (index, window) in ref_msgs.windows(2).skip(start_index).enumerate() {
        if window[0].timestamp <= timestamp && timestamp < window[1].timestamp {
            return index + start_index;
        }
    }
    ref_msgs.len() - 1
}

fn add_seconds(timestamp: DateTime<Utc>, seconds: f64) -> DateTime<Utc> {
    let secs = seconds as u64;
    let nanos = ((seconds - (secs as f64)) * 1.0e9) as u32;
    timestamp + Duration::new(secs, nanos)
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if which("ffmpeg").is_err() && !args.skip_ffmpeg {
        println!("Could not find ffmpeg. Please install it or use the --skip-ffmpeg flag.");
        return Ok(());
    }

    let colors = match args.colors {
        Some(path) => {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let colors: colors::Colors = serde_json::from_reader(reader)?;
            colors
        },
        None => colors::Colors::default()
    };

    let template = templates::get_template(&args.template)?;

    eprintln!("Loading referee messages from log...");
    let ref_messages = get_all_referee_messages(args.log_path)?;

    let font_mgr = skia_safe::FontMgr::default();

    let mut surface = templates::initialize_surface(&template, &font_mgr)?;

    let temp_dir = tempfile::Builder::new()
        .prefix("ssl_log_overlay_")
        .tempdir()?;

    let frame_dir = match args.frame_output_dir {
        Some(dir) => dir,
        None => {
            if args.skip_ffmpeg {
                eprintln!(
                    "Warning: --skip-ffmpeg is set but frames are being rendered to a temporary directory. Consider setting --frame-output-dir."
                );
            }
            temp_dir.path().to_str().unwrap().to_string()
        }
    };
    std::fs::create_dir_all(&frame_dir)?;
    eprintln!("Writing frames to {}", frame_dir);

    let frame_rate = args.frame_rate;
    let frame_duration = 1.0 / frame_rate;
    let mut frame_timestamp = ref_messages.first().unwrap().timestamp;
    let start_time = frame_timestamp;

    if let Some(start_time) = args.start_time {
        frame_timestamp = add_seconds(frame_timestamp, start_time);
    }

    let first_message_index = match args.start_time {
        Some(t) => find_ref_msg_by_time(&ref_messages, add_seconds(start_time, t), 0),
        None => 0,
    };
    let last_message_index = match args.end_time {
        Some(t) => find_ref_msg_by_time(&ref_messages, add_seconds(start_time, t), 0),
        None => ref_messages.len() - 1,
    };
    let mut current_message_index = first_message_index;

    let total_frame_count = ((ref_messages[last_message_index].timestamp - ref_messages[first_message_index].timestamp).as_seconds_f64() / frame_duration) as u64;

    eprintln!("Rendering frames...");
    let progress_bar = ProgressBar::new(total_frame_count);
    progress_bar.enable_steady_tick(Duration::from_millis(100));

    for frame_number in 0..total_frame_count {
        current_message_index =
            find_ref_msg_by_time(&ref_messages, frame_timestamp, current_message_index);

        progress_bar.set_position(frame_number);

        let message = &ref_messages[current_message_index];
        let ref_message = match &message.body {
            MessageBody::Refbox2013(msg) => Ok(msg),
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                "Unexpected message type in referee message array",
            )),
        }?;

        if current_message_index > last_message_index {
            break;
        } else if current_message_index == last_message_index
            && (frame_timestamp - message.timestamp).as_seconds_f64() > frame_duration
        {
            break;
        }

        templates::render_template_to_surface(&template, ref_message, &colors, &mut surface, &font_mgr)?;

        let frame_path = format!("{}/frame_{}.png", frame_dir, frame_number);
        let image = surface.image_snapshot();
        match image.encode(None, EncodedImageFormat::PNG, 100) {
            Some(data) => {
                std::fs::write(&frame_path, data.as_bytes())?;
            }
            None => {
                return Err(anyhow::Error::msg("Failed to encode image as PNG"));
            }
        }
        frame_timestamp = add_seconds(frame_timestamp, frame_duration);
    }

    eprintln!("Frame rendering complete.");
    progress_bar.finish_and_clear();

    if args.skip_ffmpeg {
        return Ok(());
    }

    if !args.verbose_ffmpeg_output {
        progress_bar.set_message("Combining frames with ffmpeg...");
    } else {
        progress_bar.finish_and_clear();
    }

    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(Duration::from_millis(100));

    let io = || {
        if args.verbose_ffmpeg_output {
            std::process::Stdio::inherit()
        } else {
            std::process::Stdio::null()
        }
    };

    Command::new("ffmpeg")
        .arg("-y")
        .arg("-framerate")
        .arg(format!("{}", frame_rate))
        .arg("-i")
        .arg(format!("{}/frame_%d.png", frame_dir))
        .arg("-c:v")
        .arg("png")
        .arg("-pix_fmt")
        .arg("bgra")
        .arg(args.output_path)
        .stdout(io())
        .stderr(io())
        .status()
        .expect("Failed to run ffmpeg");

    if !args.verbose_ffmpeg_output {
        spinner.finish_and_clear();
    }

    eprintln!("Video rendering complete.");

    Ok(())
}
