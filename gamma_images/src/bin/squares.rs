use image;
use image::{GenericImageView, GenericImage, ImageBuffer, RgbaImage};
use image_processing::window::*;

fn main() {

    let (image_width, image_height) = (1024, 100);
    let (window_width, window_height) = (500, 500);

    // let (image_width, image_height) = (400, 400);

    let mut three_images: RgbaImage = ImageBuffer::new(1600, 600);
    for pixel in three_images.pixels_mut() {
        pixel[0] = 255;
        pixel[1] = 255;
        pixel[2] = 255;
        pixel[3] = 255;
    }
    // let mut image: RgbaImage = ImageBuffer::new(image_width, image_height);
    // let mut second_image: RgbaImage = ImageBuffer::new(image_width, image_height);
    // let mut third_image: RgbaImage = ImageBuffer::new(image_width, image_height);

    let max: f32 = 255.0;

    let first_image_pixel = ((200.0 / max).powf(1.0 / 2.2) * max).round() as u8;
    let mut first_sub_image = three_images.sub_image(100, 100, 400, 400);

    for y in 0..first_sub_image.height() {
        for x in 0..first_sub_image.width() {
            let pixel = first_sub_image.get_pixel_mut(x, y);
            pixel[0] = first_image_pixel;
            pixel[1] = first_image_pixel;
            pixel[2] = first_image_pixel;
            pixel[3] = 255;
        }
    }

    // let second_image_pixel = ((100.0 / max).powf(1.0 / 2.2) * max).round() as u8;
    let second_image_pixel = 200;
    let mut second_sub_image = three_images.sub_image(600, 100, 400, 400);
    for y in 0..second_sub_image.height() {
        for x in 0..second_sub_image.width() {
            let pixel = second_sub_image.get_pixel_mut(x, y);
            pixel[0] = second_image_pixel;
            pixel[1] = second_image_pixel;
            pixel[2] = second_image_pixel;
            pixel[3] = 255;
        }
    }

    let third_image_pixel: f32 = ((100.0 / max).powf(1.0 / 2.2) * max).round();
    let third_image_pixel: u8 = ((third_image_pixel / max).powf(1.0 / 2.2) * max).round() as u8;
    let mut third_sub_image = three_images.sub_image(1100, 100, 400, 400);
    for y in 0..third_sub_image.height() {
        for x in 0..third_sub_image.width() {
            let pixel = third_sub_image.get_pixel_mut(x, y);
            pixel[0] = third_image_pixel;
            pixel[1] = third_image_pixel;
            pixel[2] = third_image_pixel;
            pixel[3] = 255;
        }
    }

    println!(
        "first {}\nsecond {}\nthird {}",
        first_image_pixel,
        second_image_pixel,
        third_image_pixel,
    );

    display_image("three images", &three_images, window_width, window_height);
    // three_images.save("./frontend/images/perceptual_brightness.jpeg").unwrap();
    // display_multiple_images(
    //     &["image", "second image", "third image"],
    //     &[&image, &second_image, &third_image], 
    //     window_width, 
    //     window_height
    // ); 
}
