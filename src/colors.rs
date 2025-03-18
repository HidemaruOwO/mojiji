use image::{Luma, Rgb, Rgba};

// 赤系統
pub const RED: Rgba<u8> = Rgba([255, 0, 0, 255]);
pub const DARK_RED: Rgba<u8> = Rgba([139, 0, 0, 255]);
pub const CRIMSON: Rgba<u8> = Rgba([220, 20, 60, 255]);
pub const MAROON: Rgba<u8> = Rgba([128, 0, 0, 255]);
pub const TOMATO: Rgba<u8> = Rgba([255, 99, 71, 255]);

// 青系統
pub const BLUE: Rgba<u8> = Rgba([0, 0, 255, 255]);
pub const NAVY: Rgba<u8> = Rgba([0, 0, 128, 255]);
pub const ROYAL_BLUE: Rgba<u8> = Rgba([65, 105, 225, 255]);
pub const SKY_BLUE: Rgba<u8> = Rgba([135, 206, 235, 255]);
pub const CYAN: Rgba<u8> = Rgba([0, 255, 255, 255]);
pub const TEAL: Rgba<u8> = Rgba([0, 128, 128, 255]);

// 緑系統
pub const GREEN: Rgba<u8> = Rgba([0, 128, 0, 255]);
pub const LIME: Rgba<u8> = Rgba([0, 255, 0, 255]);
pub const FOREST_GREEN: Rgba<u8> = Rgba([34, 139, 34, 255]);
pub const OLIVE: Rgba<u8> = Rgba([128, 128, 0, 255]);
pub const SPRING_GREEN: Rgba<u8> = Rgba([0, 255, 127, 255]);

// 黄系統
pub const YELLOW: Rgba<u8> = Rgba([255, 255, 0, 255]);
pub const GOLD: Rgba<u8> = Rgba([255, 215, 0, 255]);
pub const KHAKI: Rgba<u8> = Rgba([240, 230, 140, 255]);

// 紫系統
pub const PURPLE: Rgba<u8> = Rgba([128, 0, 128, 255]);
pub const VIOLET: Rgba<u8> = Rgba([238, 130, 238, 255]);
pub const INDIGO: Rgba<u8> = Rgba([75, 0, 130, 255]);
pub const MAGENTA: Rgba<u8> = Rgba([255, 0, 255, 255]);

// ピンク系統
pub const PINK: Rgba<u8> = Rgba([255, 192, 203, 255]);
pub const HOT_PINK: Rgba<u8> = Rgba([255, 105, 180, 255]);

// 茶系統
pub const BROWN: Rgba<u8> = Rgba([165, 42, 42, 255]);
pub const CHOCOLATE: Rgba<u8> = Rgba([210, 105, 30, 255]);
pub const SIENNA: Rgba<u8> = Rgba([160, 82, 45, 255]);

// モノクロ系統
pub const BLACK: Rgba<u8> = Rgba([0, 0, 0, 255]);
pub const WHITE: Rgba<u8> = Rgba([255, 255, 255, 255]);
pub const GRAY: Rgba<u8> = Rgba([128, 128, 128, 255]);
pub const LIGHT_GRAY: Rgba<u8> = Rgba([211, 211, 211, 255]);
pub const DARK_GRAY: Rgba<u8> = Rgba([169, 169, 169, 255]);

// オレンジ系統
pub const ORANGE: Rgba<u8> = Rgba([255, 165, 0, 255]);
pub const CORAL: Rgba<u8> = Rgba([255, 127, 80, 255]);
pub const SALMON: Rgba<u8> = Rgba([250, 128, 114, 255]);

const ALL_COLORS: [Rgba<u8>; 36] = [
    RED,
    DARK_RED,
    CRIMSON,
    MAROON,
    TOMATO,
    BLUE,
    NAVY,
    ROYAL_BLUE,
    SKY_BLUE,
    CYAN,
    TEAL,
    GREEN,
    LIME,
    FOREST_GREEN,
    OLIVE,
    SPRING_GREEN,
    YELLOW,
    GOLD,
    KHAKI,
    PURPLE,
    VIOLET,
    INDIGO,
    MAGENTA,
    PINK,
    HOT_PINK,
    BROWN,
    CHOCOLATE,
    SIENNA,
    BLACK,
    WHITE,
    GRAY,
    LIGHT_GRAY,
    DARK_GRAY,
    ORANGE,
    CORAL,
    SALMON,
];

// 事前定義された色からランダムに選択する関数
pub fn random() -> Rgba<u8> {
    use rand::Rng;
    let mut rng = rand::rng();
    let index = rng.random_range(0..ALL_COLORS.len());
    ALL_COLORS[index]
}

// 色の透明度を調整する
pub fn with_alpha(color: &Rgba<u8>, alpha: u8) -> Rgba<u8> {
    let mut new_color = *color;
    new_color[3] = alpha;
    new_color
}

pub fn rgba_to_rgb(rgba: &Rgba<u8>) -> Rgb<u8> {
    let [r, g, b, _] = rgba.0;
    Rgb([r, g, b])
}

pub fn rgba_to_luma(rgba: &Rgba<u8>) -> Luma<u8> {
    let r = rgba[0] as f32;
    let g = rgba[1] as f32;
    let b = rgba[2] as f32;

    let y = (0.2126 * r + 0.7152 * g + 0.0722 * b).round() as u8;

    Luma([y])
}

pub fn hex_to_rgba(hex: &str) -> Result<Rgba<u8>, String> {
    let hex = hex.trim_start_matches('#');

    let len = hex.len();
    if len != 6 && len != 8 {
        return Err(format!(
            "Invalid hex color: {} (must be 6 or 8 characters)",
            hex
        ));
    }

    let r = u8::from_str_radix(&hex[0..2], 16)
        .map_err(|_| format!("Invalid red component: {}", &hex[0..2]))?;
    let g = u8::from_str_radix(&hex[2..4], 16)
        .map_err(|_| format!("Invalid green component: {}", &hex[2..4]))?;
    let b = u8::from_str_radix(&hex[4..6], 16)
        .map_err(|_| format!("Invalid blue component: {}", &hex[4..6]))?;

    // アルファ値
    let a = if len == 8 {
        u8::from_str_radix(&hex[6..8], 16)
            .map_err(|_| format!("Invalid alpha component: {}", &hex[6..8]))?
    } else {
        255
    };

    Ok(Rgba([r, g, b, a]))
}
