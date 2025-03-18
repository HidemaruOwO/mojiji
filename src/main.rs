use rocket::State;
use text_image::{image::dynamic_image_to_vec, process::process};

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

#[get("/generate?<text>&<font>&<size>")]
async fn generate(
    text: &str,
    font: Option<&str>,
    size: Option<f32>,
    config: &State<AppConfig>,
) -> Result<Image, &'static str> {
    let font = font.unwrap_or(config.default_font);
    let size = size.unwrap_or(config.default_size);

    let image = match process(text, font, size) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    let png = dynamic_image_to_vec(&image, image::ImageOutputFormat::Png)
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
