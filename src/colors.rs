pub struct Rgba(pub [u8; 4]);

// 赤系統
pub const RED: Rgba = Rgba([255, 0, 0, 255]);
pub const DARK_RED: Rgba = Rgba([139, 0, 0, 255]);
pub const CRIMSON: Rgba = Rgba([220, 20, 60, 255]);
pub const MAROON: Rgba = Rgba([128, 0, 0, 255]);
pub const TOMATO: Rgba = Rgba([255, 99, 71, 255]);

// 青系統
pub const BLUE: Rgba = Rgba([0, 0, 255, 255]);
pub const NAVY: Rgba = Rgba([0, 0, 128, 255]);
pub const ROYAL_BLUE: Rgba = Rgba([65, 105, 225, 255]);
pub const SKY_BLUE: Rgba = Rgba([135, 206, 235, 255]);
pub const CYAN: Rgba = Rgba([0, 255, 255, 255]);
pub const TEAL: Rgba = Rgba([0, 128, 128, 255]);

// 緑系統
pub const GREEN: Rgba = Rgba([0, 128, 0, 255]);
pub const LIME: Rgba = Rgba([0, 255, 0, 255]);
pub const FOREST_GREEN: Rgba = Rgba([34, 139, 34, 255]);
pub const OLIVE: Rgba = Rgba([128, 128, 0, 255]);
pub const SPRING_GREEN: Rgba = Rgba([0, 255, 127, 255]);

// 黄系統
pub const YELLOW: Rgba = Rgba([255, 255, 0, 255]);
pub const GOLD: Rgba = Rgba([255, 215, 0, 255]);
pub const KHAKI: Rgba = Rgba([240, 230, 140, 255]);

// 紫系統
pub const PURPLE: Rgba = Rgba([128, 0, 128, 255]);
pub const VIOLET: Rgba = Rgba([238, 130, 238, 255]);
pub const INDIGO: Rgba = Rgba([75, 0, 130, 255]);
pub const MAGENTA: Rgba = Rgba([255, 0, 255, 255]);

// ピンク系統
pub const PINK: Rgba = Rgba([255, 192, 203, 255]);
pub const HOT_PINK: Rgba = Rgba([255, 105, 180, 255]);

// 茶系統
pub const BROWN: Rgba = Rgba([165, 42, 42, 255]);
pub const CHOCOLATE: Rgba = Rgba([210, 105, 30, 255]);
pub const SIENNA: Rgba = Rgba([160, 82, 45, 255]);

// モノクロ系統
pub const BLACK: Rgba = Rgba([0, 0, 0, 255]);
pub const WHITE: Rgba = Rgba([255, 255, 255, 255]);
pub const GRAY: Rgba = Rgba([128, 128, 128, 255]);
pub const LIGHT_GRAY: Rgba = Rgba([211, 211, 211, 255]);
pub const DARK_GRAY: Rgba = Rgba([169, 169, 169, 255]);

// オレンジ系統
pub const ORANGE: Rgba = Rgba([255, 165, 0, 255]);
pub const CORAL: Rgba = Rgba([255, 127, 80, 255]);
pub const SALMON: Rgba = Rgba([250, 128, 114, 255]);

// ランダムな色を返す
pub fn random() -> Rgba {
    use rand::Rng;
    let mut rng = rand::rng();
    Rgba([
        rng.random_range(0..=255),
        rng.random_range(0..=255),
        rng.random_range(0..=255),
        255,
    ])
}

pub fn with_alpha(color: &Rgba, alpha: u8) -> Rgba {
    let mut new_color = color.0;
    new_color[3] = alpha;
    Rgba(new_color)
}
