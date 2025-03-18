use ab_glyph::{FontVec, PxScale};
use image::{DynamicImage, ImageBuffer};
use imageproc::drawing::draw_text_mut;
use rusttype::{point, Font, Scale};

use crate::colors::{self, hex_to_rgba};

fn canvas_size(text: &str, text_scale: Scale, font: &Font) -> (u32, u32) {
    // テキストの実際のレンダリングサイズを計算
    let v_metrics = font.v_metrics(text_scale);

    // 行ごとに
    let lines: Vec<&str> = text.lines().collect();
    let line_count = lines.len().max(1);

    // 一番文字の長い行のながさを取得
    let max_width = lines
        .iter()
        .map(|line| {
            let glyphs: Vec<_> = font.layout(line, text_scale, point(0.0, 0.0)).collect();

            glyphs
                .last()
                .and_then(|g| g.pixel_bounding_box().map(|bb| bb.max.x as f32))
                .unwrap_or(0.0)
        })
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap_or(0.0);

    // 高さの計算
    let height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
    let total_height = height * line_count as f32;

    (max_width.ceil() as u32, total_height.ceil() as u32)
}

pub fn process(
    text: &str,
    font: &str,
    size: f32,
    color: &str,
) -> Result<DynamicImage, &'static str> {
    let text_scale = Scale { x: size, y: size };

    // フォントを読み込む
    let font_data = match font {
        "Noto" => include_bytes!("../../fonts/NotoSansJP-Bold.ttf"),
        _ => return Err("Unsupported font."),
    };
    let font_raw = match Font::try_from_bytes(font_data) {
        Some(v) => v,
        None => return Err("Failed to load fonts."),
    };
    let font_source = match FontVec::try_from_vec(Vec::from(font_data as &[u8])) {
        Ok(v) => v,
        Err(_) => return Err("Failed to load fonts."),
    };

    // 元の画像を定義
    let (width, height) = canvas_size(text, text_scale, &font_raw);
    let color = if color == "random" {
        colors::random()
    } else {
        match hex_to_rgba(color) {
            Ok(v) => v,
            Err(_) => return Err("Failed to convert hex to rgba."),
        }
    };

    let mut image_buffer = ImageBuffer::new(width, height);

    for (i, line) in text.lines().enumerate() {
        // センタリングのため
        let line_width = {
            let glyphs: Vec<_> = font_raw.layout(line, text_scale, point(0.0, 0.0)).collect();
            glyphs
                .last()
                .and_then(|g| g.pixel_bounding_box().map(|bb| bb.max.x as f32))
                .unwrap_or(0.0)
                .ceil() as u32
        };

        let x_position = (width - line_width) / 2;

        draw_text_mut(
            &mut image_buffer,
            color,
            x_position.try_into().unwrap(), // センタリング。ここが 0 だとセンタリングをしない
            ((size * (i as f32)) as u32).try_into().unwrap(), // 行で分けるように
            PxScale::from(size),
            &font_source,
            line,
        );
    }

    Ok(DynamicImage::ImageRgba8(image_buffer))
}
