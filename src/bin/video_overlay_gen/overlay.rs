use crate::text_helpers::{limit_str_length, draw_text_centered};
use ssl_logtools_rs::protos::refbox::ssl_gc_referee_message::{Referee, referee};
use skia_safe::{Canvas, Color, Paint, Rect, Surface, Typeface, ISize, surfaces};

const BLUE: Color = Color::from_rgb(168, 168, 255);
const YELLOW: Color = Color::from_rgb(255, 255, 168);
const RED: Color = Color::from_rgb(255, 145, 145);
const GREEN: Color = Color::from_rgb(168, 255, 168);

const SIZE: ISize = ISize::new(1920, 300);

pub fn create_surface() -> Option<Surface> {
    surfaces::raster_n32_premul(SIZE)
}

pub fn draw_overlay(surface: &mut Surface, ref_message: &Referee, typeface: &Typeface) {
    draw_background(surface);
    draw_team_info(surface.canvas(), ref_message, typeface);
    draw_game_command(surface.canvas(), ref_message, typeface);
    draw_game_time(surface.canvas(), ref_message, typeface);
    draw_game_stage(surface.canvas(), ref_message, typeface);
}

fn draw_background(surface: &mut Surface) {
    surface.canvas().clear(Color::TRANSPARENT);
    let rect = Rect::new(0.0, 0.0, surface.width() as f32, surface.height() as f32);
    let corner_radius = 50.0;
    let mut paint = Paint::default();
    paint.set_color(Color::DARK_GRAY);
    paint.set_anti_alias(true);
    surface
        .canvas()
        .draw_round_rect(rect, corner_radius, corner_radius, &paint);
}

fn draw_team_info(canvas: &Canvas, ref_message: &Referee, typeface: &Typeface) {
    let mut blue_paint = skia_safe::Paint::default();
    blue_paint.set_color(BLUE);
    blue_paint.set_anti_alias(true);

    let mut yellow_paint = skia_safe::Paint::default();
    yellow_paint.set_color(YELLOW);
    yellow_paint.set_anti_alias(true);

    let y_team_name = &limit_str_length(ref_message.yellow.name(), 15);
    let b_team_name = &limit_str_length(ref_message.blue.name(), 15);
    let name_font = skia_safe::Font::new(typeface, 64.0);
    draw_text_centered(
        &canvas,
        y_team_name,
        (300, 70),
        &name_font,
        &yellow_paint,
    );
    draw_text_centered(
        &canvas,
        b_team_name,
        (1620, 70),
        &name_font,
        &blue_paint,
    );

    let score_font = skia_safe::Font::new(typeface, 218.0);
    let y_team_score = &format!("{}", ref_message.yellow.score());
    let b_team_score = &format!("{}", ref_message.blue.score());
    draw_text_centered(
        &canvas,
        y_team_score,
        (300, 270),
        &score_font,
        &yellow_paint,
    );
    draw_text_centered(
        &canvas,
        b_team_score,
        (1620, 270),
        &score_font,
        &blue_paint,
    );
}

fn draw_game_command(canvas: &Canvas, ref_message: &Referee, typeface: &Typeface) {
    let mut paint = skia_safe::Paint::default();
    paint.set_color(match ref_message.command() {
        referee::Command::HALT => RED,
        referee::Command::STOP => RED,
        referee::Command::NORMAL_START => GREEN,
        referee::Command::FORCE_START => GREEN,
        referee::Command::PREPARE_KICKOFF_YELLOW => YELLOW,
        referee::Command::PREPARE_KICKOFF_BLUE => BLUE,
        referee::Command::PREPARE_PENALTY_YELLOW => YELLOW,
        referee::Command::PREPARE_PENALTY_BLUE => BLUE,
        referee::Command::DIRECT_FREE_YELLOW => YELLOW,
        referee::Command::DIRECT_FREE_BLUE => BLUE,
        referee::Command::INDIRECT_FREE_YELLOW => YELLOW,
        referee::Command::INDIRECT_FREE_BLUE => BLUE,
        referee::Command::TIMEOUT_YELLOW => YELLOW,
        referee::Command::TIMEOUT_BLUE => BLUE,
        referee::Command::GOAL_YELLOW => YELLOW,
        referee::Command::GOAL_BLUE => BLUE,
        referee::Command::BALL_PLACEMENT_YELLOW => YELLOW,
        referee::Command::BALL_PLACEMENT_BLUE => BLUE,
    });
    paint.set_anti_alias(true);

    let command_text = match ref_message.command() {
        referee::Command::HALT => "HALT",
        referee::Command::STOP => "STOP",
        referee::Command::NORMAL_START => "NORMAL START",
        referee::Command::FORCE_START => "FORCE START",
        referee::Command::PREPARE_KICKOFF_YELLOW => "KICKOFF",
        referee::Command::PREPARE_KICKOFF_BLUE => "KICKOFF",
        referee::Command::PREPARE_PENALTY_YELLOW => "PREPARE PENALTY",
        referee::Command::PREPARE_PENALTY_BLUE => "PREPARE PENALTY",
        referee::Command::DIRECT_FREE_YELLOW => "DIRECT FREE",
        referee::Command::DIRECT_FREE_BLUE => "DIRECT FREE",
        referee::Command::INDIRECT_FREE_YELLOW => "INDIRECT FREE",
        referee::Command::INDIRECT_FREE_BLUE => "INDIRECT FREE",
        referee::Command::TIMEOUT_YELLOW => "TIMEOUT",
        referee::Command::TIMEOUT_BLUE => "TIMEOUT",
        referee::Command::GOAL_YELLOW => "GOAL",
        referee::Command::GOAL_BLUE => "GOAL",
        referee::Command::BALL_PLACEMENT_YELLOW => "BALL PLACEMENT",
        referee::Command::BALL_PLACEMENT_BLUE => "BALL PLACEMENT",
    };

    let command_font = skia_safe::Font::new(typeface, 64.0);
    draw_text_centered(
        &canvas,
        command_text,
        (960, 270),
        &command_font,
        &paint,
    );
}

fn draw_game_stage(canvas: &Canvas, ref_message: &Referee, typeface: &Typeface) {
    let mut paint = skia_safe::Paint::default();
    paint.set_color(Color::WHITE);
    paint.set_anti_alias(true);

    let stage_text = match ref_message.stage() {
        referee::Stage::NORMAL_FIRST_HALF_PRE => "PREPARE FIRST HALF",
        referee::Stage::NORMAL_FIRST_HALF => "FIRST HALF",
        referee::Stage::NORMAL_HALF_TIME => "HALF TIME",
        referee::Stage::NORMAL_SECOND_HALF_PRE => "PREPARE SECOND HALF",
        referee::Stage::NORMAL_SECOND_HALF => "SECOND HALF",
        referee::Stage::EXTRA_TIME_BREAK => "EXTRA TIME",
        referee::Stage::EXTRA_FIRST_HALF_PRE => "PREPARE FIRST EXTRA HALF",
        referee::Stage::EXTRA_FIRST_HALF => "FIRST EXTRA HALF",
        referee::Stage::EXTRA_HALF_TIME => "HALF TIME",
        referee::Stage::EXTRA_SECOND_HALF_PRE => "PREPARE SECOND EXTRA HALF",
        referee::Stage::EXTRA_SECOND_HALF => "SECOND EXTRA HALF",
        referee::Stage::PENALTY_SHOOTOUT_BREAK => "PENALTY SHOOTOUT",
        referee::Stage::PENALTY_SHOOTOUT => "PENALTY SHOOTOUT",
        referee::Stage::POST_GAME => "POST GAME",
    };

    let stage_font = skia_safe::Font::new(typeface, 64.0);
    draw_text_centered(&canvas, stage_text, (960, 70), &stage_font, &paint);
}

fn draw_game_time(canvas: &Canvas, ref_message: &Referee, typeface: &Typeface) {
    let mut paint = skia_safe::Paint::default();
    paint.set_color(match ref_message.command() {
        referee::Command::HALT => RED,
        referee::Command::STOP => RED,
        _ => Color::WHITE,
    });
    paint.set_anti_alias(true);

    if let Some(stage_time_left_micros) = ref_message.stage_time_left {
        let stage_time_left_minutes = (stage_time_left_micros / 1_000_000 / 60).abs();
        let stage_time_left_seconds = ((stage_time_left_micros / 1_000_000) % 60).abs();
        let prefix = if stage_time_left_micros < 0 { "-" } else { "" };
        let time_text = format!(
            "{}{}:{:0>2}",
            prefix, stage_time_left_minutes, stage_time_left_seconds
        );
        let time_font = skia_safe::Font::new(typeface, 122.0);
        draw_text_centered(&canvas, &time_text, (960, 190), &time_font, &paint);
    }
}
