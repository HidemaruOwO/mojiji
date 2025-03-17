use image::{DynamicImage, ImageBuffer, Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::{point, Font, Scale};

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

pub fn process(text: &str, font: &str) -> Result<DynamicImage, &'static str> {
    let text_size = 250.0;
    let text_scale = Scale {
        x: text_size,
        y: text_size,
    };

    // フォントを読み込む
    let font_data = match font {
        "Noto" => include_bytes!("../../fonts/NotoSansJP-Bold.ttf"),
        _ => return Err("Unsupported font."),
    };
    let font_raw = match Font::try_from_vec(Vec::from(font_data as &[u8])) {
        Some(v) => v,
        None => return Err("Failed to load fonts."),
    };

    // 元の画像を定義
    let (width, height) = canvas_size(text, text_scale, &font_raw);
    let mut image = DynamicImage::ImageRgba8(ImageBuffer::from_fn(width, height, |_x, _y| {
        Rgba([0, 0, 0, 0])
    }));

    // テキストとフォントをレンダリング
    draw_text_mut(
        &mut image,
        Rgba([255, 255, 255, 255]),
        0,
        0,
        text_scale,
        &font_raw,
        text,
    );

    Ok(image)
}
