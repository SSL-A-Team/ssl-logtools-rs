use chrono::{DateTime, Utc};
use clap::Parser;
use ssl_loglib::MessageBody;
use ssl_loglib::log_file_reader::LogFileReader;
use ssl_loglib::raw::MessageType;
use std::collections::HashMap;
use std::io;

#[derive(Parser)]
#[command(version)]
struct Args {
    log_path: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let reader = LogFileReader::new(args.log_path)?;

    let mut counts: HashMap<MessageType, u32> = HashMap::new();

    let mut first_timestamp: Option<DateTime<Utc>> = None;
    let mut last_timestamp: Option<DateTime<Utc>> = None;

    for message in reader {
        let message_type = match message.body {
            MessageBody::Blank(_) => MessageType::Blank,
            MessageBody::Unkown(_) => MessageType::Unkown,
            MessageBody::Vision2010(_) => MessageType::Vision2010,
            MessageBody::Refbox2013(_) => MessageType::Refbox2013,
            MessageBody::Vision2014(_) => MessageType::Vision2014,
            MessageBody::VisionTracker2020(_) => MessageType::VisionTracker2020,
            MessageBody::Index2021(_) => MessageType::Index2021,
        };
        match counts.get_mut(&message_type) {
            Some(c) => {
                *c += 1;
            }
            None => {
                counts.insert(message_type, 1);
            }
        };
        if first_timestamp.is_none() {
            first_timestamp = Some(message.timestamp);
        }
        last_timestamp = Some(message.timestamp);
    }

    if counts.is_empty() {
        println!("No messages found in log file.");
        return Ok(());
    }

    if let Some(first_time) = first_timestamp
        && let Some(last_time) = last_timestamp
    {
        let log_duration = (last_time - first_time).as_seconds_f64();
        println!("Log duration: {}s", log_duration);
        for (t, c) in counts {
            let type_string = format!("{:?}", t);
            println!("{: <20} {} msgs  ({: >7.2} Hz)", type_string, c, (c as f64 / log_duration));
        }
    }

    Ok(())
}
