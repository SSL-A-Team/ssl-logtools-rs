use skia_safe::{Canvas, Font, Paint, Point};

pub fn draw_text_centered(canvas: &Canvas, text: &str, origin: impl Into<Point>, font: &Font, paint: &Paint) {
    let text_width = font.measure_str(text, Some(&paint)).1.width();
    let origin = origin.into();
    let x = origin.x;
    let y = origin.y;
    let adjusted_x = x - (text_width / 2.0);
    canvas.draw_str(text, (adjusted_x, y), font, &paint);
}

pub fn limit_str_length(text: &str, max_length: usize) -> String {
    if text.len() <= max_length {
        text.to_string()
    } else {
        format!("{}...", &text[..(max_length - 3)])
    }
}
