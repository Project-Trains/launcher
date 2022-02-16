use image;
use image::GenericImageView;

pub async fn load_img(url: &str) -> (Vec<u8>, u32, u32) {
    let resp = reqwest::get(url).await.unwrap();
    let img = image::load_from_memory(resp.bytes().await.unwrap().as_ref()).unwrap();
    let (w, h) = img.dimensions();
    (img.as_bytes().to_vec(), w, h)
}

pub fn parse_img(img_bytes: Vec<u8>, w: u32, h: u32) -> sixtyfps::Image {
    let mut pixel_buffer = sixtyfps::SharedPixelBuffer::new(w as usize, h as usize);
    let bytes = pixel_buffer.make_mut_bytes();
    
    // print!("w * h: {0}\n", w * h);
    // print!("img_bytes: {0}\n", img_bytes.len());
    // print!("bytes: {0}\n", bytes.len());
    // print!("\n");
    
    for i in 0..img_bytes.len() {
        // Index out of bounds check
        // if i >= bytes.len() || i >= img_bytes.len() {
        //     break;
        // }

        bytes[i] = img_bytes[i];
    }

    sixtyfps::Image::from_rgba8(pixel_buffer)
}
