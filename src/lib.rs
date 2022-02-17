use lodepng::decode32;
use rgb::ComponentBytes;

pub async fn load_img(url: &str) -> (Vec<u8>, usize, usize) {
    let resp = tinyget::get(url).send().unwrap();
    let bytes = resp.as_bytes();
    let img = decode32(bytes).unwrap();
    (img.buffer.as_bytes().to_vec(), img.width, img.height)
}

pub fn parse_img(img_bytes: Vec<u8>, w: usize, h: usize) -> slint::Image {
    let mut pixel_buffer = slint::SharedPixelBuffer::new(w as u32, h as u32);
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

    slint::Image::from_rgba8(pixel_buffer)
}
