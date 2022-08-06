use super::{
    filters::{BadAuth, ColorJson, InvalidColor, OutOfBounds},
    models::Image,
    HEIGHT, WIDTH,
};

use mtpng::{
    encoder::{Encoder, Options},
    ColorType, Header,
};
use serde::Serialize;
use std::convert::Infallible;
use warp::{
    http::{Response, StatusCode},
    Rejection, Reply,
};

#[derive(Serialize)]
struct Message {
    code: u16,
    message: String,
}

pub async fn information() -> Result<impl Reply, Infallible> {
    Ok(reply_with_message(
        StatusCode::OK,
        "Welcome to r/place but in Rust!".to_string(),
    ))
}

pub async fn get_image(image: Image) -> Result<impl Reply, Infallible> {
    let image = image.lock().await;
    let buffer = Vec::<u8>::new();

    let mut header = Header::new();
    header.set_size(WIDTH as u32, HEIGHT as u32).unwrap();
    header.set_color(ColorType::Truecolor, 8).unwrap();

    let options = Options::new();
    let mut encoder = Encoder::new(buffer, &options);

    encoder.write_header(&header).unwrap();
    encoder.write_image_rows(&image).unwrap();
    let buffer = encoder.finish().unwrap();

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/png")
        .body(buffer))
}

pub async fn set_pixel(
    x: usize,
    y: usize,
    query: ColorJson,
    image: Image,
) -> Result<impl Reply, Rejection> {
    if x >= WIDTH || y >= HEIGHT {
        return Err(warp::reject::custom(OutOfBounds));
    }

    let color = match hex_to_rgb(&query.color) {
        Err(_) => return Err(warp::reject::custom(InvalidColor)),
        Ok(color) => color,
    };

    let mut image = image.lock().await;
    let i = (x + y * WIDTH) * 3;

    image[i] = color.0;
    image[i + 1] = color.1;
    image[i + 2] = color.2;

    Ok(reply_with_message(
        StatusCode::OK,
        "Successfully edited pixel".to_string(),
    ))
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code: StatusCode;
    let message: String;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND".to_string();
    } else if let Some(_) = err.find::<warp::reject::LengthRequired>() {
        code = StatusCode::LENGTH_REQUIRED;
        message = "MISSING_CONTENT_LENGTH".to_string();
    } else if let Some(_) = err.find::<warp::reject::PayloadTooLarge>() {
        code = StatusCode::PAYLOAD_TOO_LARGE;
        message = "PAYLOAD_TOO_LARGE".to_string();
    } else if let Some(_) = err.find::<warp::body::BodyDeserializeError>() {
        code = StatusCode::UNPROCESSABLE_ENTITY;
        message = "MALFORMED_BODY".to_string();
    } else if let Some(e) = err.find::<warp::reject::MissingHeader>() {
        code = StatusCode::BAD_REQUEST;
        message = format!("MISSING_HEADER: {}", e.name());
    } else if let Some(_) = err.find::<BadAuth>() {
        code = StatusCode::UNAUTHORIZED;
        message = "BAD_AUTH".to_string();
    } else if let Some(_) = err.find::<OutOfBounds>() {
        code = StatusCode::UNPROCESSABLE_ENTITY;
        message = "OUT_OF_BOUNDS".to_string();
    } else if let Some(_) = err.find::<InvalidColor>() {
        code = StatusCode::UNPROCESSABLE_ENTITY;
        message = "INVALID_COLOR".to_string();
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED".to_string();
    } else {
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION".to_string();
    }

    Ok(reply_with_message(code, message))
}

fn reply_with_message(code: StatusCode, message: String) -> impl Reply {
    let json = warp::reply::json(&Message {
        code: code.as_u16(),
        message,
    });

    warp::reply::with_status(json, code)
}

fn hex_to_rgb(s: &str) -> Result<(u8, u8, u8), ()> {
    if s.len() != 6 {
        Err(())
    } else {
        match u32::from_str_radix(s, 16) {
            Err(_) => Err(()),
            Ok(hex) => {
                let hex = hex as i32;
                Ok((
                    ((hex >> 16) & 255) as u8,
                    ((hex >> 8) & 255) as u8,
                    (hex & 255) as u8,
                ))
            }
        }
    }
}
