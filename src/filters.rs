use super::{handlers, models::Image};

use serde::{Deserialize, Serialize};
use warp::{Filter, Rejection, Reply};

#[derive(Deserialize, Serialize)]
pub struct ColorJson {
    pub color: String,
}

#[derive(Debug)]
pub struct BadAuth;
impl warp::reject::Reject for BadAuth {}

#[derive(Debug)]
pub struct OutOfBounds;
impl warp::reject::Reject for OutOfBounds {}

#[derive(Debug)]
pub struct InvalidColor;
impl warp::reject::Reject for InvalidColor {}

/// All filters combined
pub fn routes(image: Image) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    information()
        .or(get_image(image.clone()))
        .or(set_pixel(image))
}

/// GET /
fn information() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path::end()
        .and(warp::get())
        .and_then(handlers::information)
}

/// GET /image
fn get_image(image: Image) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("image")
        .and(warp::get())
        .and(with_image(image))
        .and_then(handlers::get_image)
}

/// POST set-pixel/:x/:y
fn set_pixel(image: Image) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("set-pixel" / usize / usize)
        .and(warp::put())
        .and(check_auth())
        .and(warp::body::content_length_limit(32).and(warp::body::json::<ColorJson>()))
        .and(with_image(image))
        .and_then(handlers::set_pixel)
}

fn check_auth() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    warp::header::<String>("X-Token")
        .and_then(|token| async move {
            if token != "abc" {
                Err(warp::reject::custom(BadAuth))
            } else {
                Ok(())
            }
        })
        .untuple_one()
}

fn with_image(
    image: Image,
) -> impl Filter<Extract = (Image,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || image.clone())
}
