pub fn render(img_path: &str) -> image::GrayImage {
    // Open an image file
    let img = image::open(img_path).unwrap();

    // Convert the image to grayscale
    let gray_img = convert_to_grayscale(&img);

    gray_img
}

fn convert_to_grayscale(image: &image::DynamicImage) -> image::GrayImage {
    // Convert the image to grayscale
    let gray_img = image.to_luma8();
    
    gray_img
}