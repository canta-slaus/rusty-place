use super::SIZE;

use std::{fs, sync::Arc};
use tokio::sync::Mutex;

pub type Image = Arc<Mutex<Vec<u8>>>;

pub fn load_from_file(path: &str) -> Image {
    match fs::read(path) {
        Ok(bytes) => Arc::new(Mutex::new(bytes)),
        Err(_) => {
            println!("Couldn't access save file");
            Arc::new(Mutex::new(vec![0x00; SIZE]))
        }
    }
}

pub async fn write_to_file(path: &str, image: &Image) {
    let vec = image.lock().await.to_vec();

    match fs::write(path, vec) {
        Ok(()) => {}
        Err(e) => {
            println!("Couldn't save the image data");
            println!("{}", e);
        }
    }
}
