use std::fs;

// TODO: return an error code
pub fn write_image_to_file(filepath: String, img_buf: &Vec<u8>, width: usize, height: usize) {
    if !fs::metadata("output").is_ok() {
        fs::create_dir("output").unwrap();
    }

    image::save_buffer(filepath, img_buf, width as u32, height as u32, image::ColorType::Rgb8).unwrap();

}