use rocket::State;
use text_image::{image::dynamic_image_to_vec, process::process};

mod colors;
mod text_image;

#[macro_use]
extern crate rocket;

struct AppConfig {
    default_font: &'static str,
    default_size: f32,
}

#[derive(Responder)]
#[response(content_type = "image/png")]
struct Image(Vec<u8>);

#[get("/")]
fn index() -> &'static str {
    "mojiji is running now."
}

#[get("/generate?<text>&<font>&<size>&<color>")]
async fn generate(
    text: &str,
    color: Option<&str>,
    font: Option<&str>,
    size: Option<f32>,
    config: &State<AppConfig>,
) -> Result<Image, &'static str> {
    let text = &text.replace("\\n", "\n");
    let color = color.unwrap_or("random");

    let font = font.unwrap_or(config.default_font);
    let mut size = size.unwrap_or(config.default_size);

    // サイズが大きすぎたら処理が固まるので 500サイズまで
    if size > 500.0 {
        size = 100.0;
    }

    let image = match process(text, font, size, color) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    let png = dynamic_image_to_vec(&image, image::ImageFormat::Png)
        .expect("Failed to generate png image.");

    Ok(Image(png))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(AppConfig {
            default_font: "Noto",
            default_size: 100.0,
        })
        .mount("/", routes![index, generate])
}
