use lodepng::decode32;
use rgb::ComponentBytes;
use slint::{Image, SharedPixelBuffer};

pub mod post;

pub async fn load_img(url: &str) -> (Vec<u8>, usize, usize) {
    let resp = tinyget::get(url).send().unwrap();
    let bytes = resp.as_bytes();
    let img = decode32(bytes).unwrap();
    (img.buffer.as_bytes().to_vec(), img.width, img.height)
}

pub fn parse_img(img_bytes: Vec<u8>, w: usize, h: usize) -> Image {
    let mut pixel_buffer = SharedPixelBuffer::new(w as u32, h as u32);
    let bytes = pixel_buffer.make_mut_bytes();
    for i in 0..img_bytes.len() {
        bytes[i] = img_bytes[i];
    }

    Image::from_rgba8(pixel_buffer)
}
