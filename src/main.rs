use text_image::{image::dynamic_image_to_vec, process::process};

mod text_image;

#[macro_use]
extern crate rocket;

#[derive(Responder)]
#[response(content_type = "image/png")]
struct Image(Vec<u8>);

#[get("/")]
fn index() -> &'static str {
    "mojiji is running now."
}

#[get("/generate")]
fn generate() -> Result<Image, &'static str> {
    // TODO
    // サイズとフォントと文字は全てパラメータから持って来れるようにする
    // /generate?text=おはよう&font=Noto&size=100
    // パラメータのsizeはf32
    let image = match process("おはようございます", "Noto", 100.0) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    let png = dynamic_image_to_vec(&image, image::ImageOutputFormat::Png)
        .expect("Failed to generate png image.");

    Ok(Image(png))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, generate])
}
