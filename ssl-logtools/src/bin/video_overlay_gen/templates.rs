use crate::colors::Colors;
use include_dir::{Dir, DirEntry, include_dir};
use skia_safe::resources::NativeResourceProvider;
use skia_safe::{Color, FontMgr, Surface, surfaces, svg};
use ssl_loglib::protos::refbox::ssl_gc_referee_message::Referee;
use ssl_loglib::protos::refbox::ssl_gc_referee_message::referee::{Command, Stage};
use tera::Tera;

const BUILTIN_TEMPLATES: Dir =
    include_dir!("$CARGO_MANIFEST_DIR/src/bin/video_overlay_gen/templates");

pub fn get_template(template_name: &str) -> anyhow::Result<Tera> {
    match BUILTIN_TEMPLATES.get_entry(template_name.to_string() + ".svg") {
        Some(DirEntry::File(file)) => {
            let content = file
                .contents_utf8()
                .ok_or(anyhow::Error::msg("Failed to read builtin template file"))?;
            let mut tera = Tera::default();
            tera.add_raw_template(template_name, content)?;
            Ok(tera)
        }
        _ => Ok(Tera::new(template_name)?),
    }
}

pub fn initialize_surface(template: &Tera, font_mgr: &FontMgr) -> anyhow::Result<Surface> {
    let svg_content = render_template(template, &Referee::default(), &Colors::default())?;
    let svg_dom = svg::Dom::from_str(svg_content, NativeResourceProvider::from(font_mgr.clone()))?;
    let size = svg_dom.root().intrinsic_size();
    surfaces::raster_n32_premul((size.width as i32, size.height as i32))
        .ok_or(anyhow::Error::msg("Failed to create surface"))
}

pub fn render_template_to_surface(
    template: &Tera,
    ref_message: &Referee,
    colors: &Colors,
    surface: &mut Surface,
    font_mgr: &FontMgr,
) -> anyhow::Result<()> {
    let svg_content = render_template(template, ref_message, colors)?;
    let mut svg_dom =
        svg::Dom::from_str(svg_content, NativeResourceProvider::from(font_mgr.clone()))?;
    let info = surface.image_info();
    svg_dom.set_container_size((info.width(), info.height()));
    surface.canvas().clear(Color::TRANSPARENT);
    svg_dom.render(surface.canvas());
    Ok(())
}

fn render_template(
    template: &Tera,
    ref_message: &Referee,
    colors: &Colors,
) -> anyhow::Result<String> {
    let mut context = tera::Context::new();

    let mut yellow_map = tera::Map::new();
    yellow_map.insert(
        "name".to_string(),
        tera::Value::String(ref_message.yellow.name().to_string()),
    );
    yellow_map.insert(
        "score".to_string(),
        tera::Value::Number(ref_message.yellow.score().into()),
    );
    context.insert("yellow", &yellow_map);

    let mut blue_map = tera::Map::new();
    blue_map.insert(
        "name".to_string(),
        tera::Value::String(ref_message.blue.name().to_string()),
    );
    blue_map.insert(
        "score".to_string(),
        tera::Value::Number(ref_message.blue.score().into()),
    );
    context.insert("blue", &blue_map);

    context.insert("stage", stage_to_string(&ref_message.stage()));
    context.insert("command", command_to_string(&ref_message.command()));
    context.insert(
        "command_color",
        command_color(&ref_message.command(), colors),
    );
    context.insert("clock_color", clock_color(&ref_message.command(), colors));

    let stage_time_minutes = match ref_message.stage_time_left {
        Some(time_left_micros) => format!("{:0>2}", (time_left_micros / 1_000_000 / 60).abs()),
        None => "".to_string(),
    };
    let stage_time_seconds = match ref_message.stage_time_left {
        Some(time_left_micros) => format!("{:0>2}", ((time_left_micros / 1_000_000) % 60).abs()),
        None => "".to_string(),
    };
    let stage_time_negative = match ref_message.stage_time_left {
        Some(time_left_micros) => {
            if time_left_micros < 0 {
                "-".to_string()
            } else {
                "".to_string()
            }
        }
        None => "".to_string(),
    };
    context.insert("stage_time_minutes", &stage_time_minutes);
    context.insert("stage_time_seconds", &stage_time_seconds);
    context.insert("stage_time_negative", &stage_time_negative);

    Ok(template.render(template.get_template_names().next().unwrap(), &context)?)
}

fn command_to_string(command: &Command) -> &'static str {
    match command {
        Command::HALT => "HALT",
        Command::STOP => "STOP",
        Command::NORMAL_START => "NORMAL START",
        Command::FORCE_START => "FORCE START",
        Command::PREPARE_KICKOFF_YELLOW => "KICKOFF",
        Command::PREPARE_KICKOFF_BLUE => "KICKOFF",
        Command::PREPARE_PENALTY_YELLOW => "PREPARE PENALTY",
        Command::PREPARE_PENALTY_BLUE => "PREPARE PENALTY",
        Command::DIRECT_FREE_YELLOW => "DIRECT FREE",
        Command::DIRECT_FREE_BLUE => "DIRECT FREE",
        Command::INDIRECT_FREE_YELLOW => "INDIRECT FREE",
        Command::INDIRECT_FREE_BLUE => "INDIRECT FREE",
        Command::TIMEOUT_YELLOW => "TIMEOUT",
        Command::TIMEOUT_BLUE => "TIMEOUT",
        Command::GOAL_YELLOW => "GOAL",
        Command::GOAL_BLUE => "GOAL",
        Command::BALL_PLACEMENT_YELLOW => "BALL PLACEMENT",
        Command::BALL_PLACEMENT_BLUE => "BALL PLACEMENT",
    }
}

fn command_color<'a>(command: &Command, colors: &'a Colors) -> &'a String {
    match command {
        Command::HALT => &colors.command_stop,
        Command::STOP => &colors.command_stop,
        Command::NORMAL_START => &colors.command_start,
        Command::FORCE_START => &colors.command_start,
        Command::PREPARE_KICKOFF_YELLOW => &colors.command_yellow_team,
        Command::PREPARE_KICKOFF_BLUE => &colors.command_blue_team,
        Command::PREPARE_PENALTY_YELLOW => &colors.command_yellow_team,
        Command::PREPARE_PENALTY_BLUE => &colors.command_blue_team,
        Command::DIRECT_FREE_YELLOW => &colors.command_yellow_team,
        Command::DIRECT_FREE_BLUE => &colors.command_blue_team,
        Command::INDIRECT_FREE_YELLOW => &colors.command_yellow_team,
        Command::INDIRECT_FREE_BLUE => &colors.command_blue_team,
        Command::TIMEOUT_YELLOW => &colors.command_yellow_team,
        Command::TIMEOUT_BLUE => &colors.command_blue_team,
        Command::GOAL_YELLOW => &colors.command_yellow_team,
        Command::GOAL_BLUE => &colors.command_blue_team,
        Command::BALL_PLACEMENT_YELLOW => &colors.command_yellow_team,
        Command::BALL_PLACEMENT_BLUE => &colors.command_blue_team,
    }
}

fn clock_color<'a>(command: &Command, colors: &'a Colors) -> &'a String {
    match command {
        Command::HALT => &colors.clock_stopped,
        Command::STOP => &colors.clock_stopped,
        _ => &colors.clock_running,
    }
}

fn stage_to_string(stage: &Stage) -> &'static str {
    match stage {
        Stage::NORMAL_FIRST_HALF_PRE => "PREPARE FIRST HALF",
        Stage::NORMAL_FIRST_HALF => "FIRST HALF",
        Stage::NORMAL_HALF_TIME => "HALF TIME",
        Stage::NORMAL_SECOND_HALF_PRE => "PREPARE SECOND HALF",
        Stage::NORMAL_SECOND_HALF => "SECOND HALF",
        Stage::EXTRA_TIME_BREAK => "EXTRA TIME",
        Stage::EXTRA_FIRST_HALF_PRE => "PREPARE FIRST EXTRA HALF",
        Stage::EXTRA_FIRST_HALF => "FIRST EXTRA HALF",
        Stage::EXTRA_HALF_TIME => "HALF TIME",
        Stage::EXTRA_SECOND_HALF_PRE => "PREPARE SECOND EXTRA HALF",
        Stage::EXTRA_SECOND_HALF => "SECOND EXTRA HALF",
        Stage::PENALTY_SHOOTOUT_BREAK => "PENALTY SHOOTOUT BREAK",
        Stage::PENALTY_SHOOTOUT => "PENALTY SHOOTOUT",
        Stage::POST_GAME => "POST GAME",
    }
}
