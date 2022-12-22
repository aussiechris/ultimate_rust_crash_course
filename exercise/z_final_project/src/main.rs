// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

use std::u8;

use clap::{Parser, Subcommand, ValueEnum};
use image::DynamicImage;

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Args {
    ///
    #[command(subcommand)]
    command: Commands,
    /// input image file
    // #[arg(value_name = "INPUT_FILE")]
    infile: String,
    /// output image file
    // #[arg(value_name = "OUTPUT_FILE")]
    outfile: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Blur the image
    Blur {
        /// amount to blur by
        #[arg(value_name = "BLUR_AMOUNT")]
        blur_amount: f32,
    },
    /// Make the image brighter
    Brighten {
        /// amount to brighten by
        #[arg(value_name = "BRIGHTEN_AMOUNT")]
        brighten_amount: i32,
    },
    /// Crop the image
    Crop {
        /// x position to crop image from
        #[arg(value_name = "CROP_X")]
        x: u32,
        /// y position to crop image from
        #[arg(value_name = "CROP_Y")]
        y: u32,
        /// width to crop image to
        #[arg(value_name = "CROP_WIDTH")]
        width: u32,
        /// height to crop image to
        #[arg(value_name = "CROP_HEIGHT")]
        height: u32,
    },
    /// Rotate the image
    Rotate {
        /// amount to rotate by
        #[arg(value_name = "ROTATE_AMOUNT")]
        rotate_amount: RotateAmount,
    },
    /// Invert the image
    Invert,
    /// Remove colour from the image
    Grayscale,
    /// Generate a fun image
    Generate {
        /// Amount of Red in generated colour swatch (0-255)
        #[arg(value_name = "RED_AMOUNT")]
        red_amount: u8,
        /// Amount of Green in generated colour swatch (0-255)
        #[arg(value_name = "GREEN_AMOUNT")]
        green_amount: u8,
        /// Amount of Blue in generated colour swatch (0-255)
        #[arg(value_name = "BLUE_AMOUNT")]
        blue_amount: u8,
    },
    /// Generate a fractal
    Fractal,
}

#[derive(ValueEnum, Clone)]
enum RotateAmount {
    /// rotate 90 degrees
    Right,
    /// rotate 180 degrees
    Flip,
    /// rotate 270 degrees
    Left,
}

fn main() {
    let args = Args::parse();

    // open the image
    let mut img = image::open(args.infile).expect("Failed to open INFILE.");

    // process the image
    match args.command {
        Commands::Blur { blur_amount } => img = blur(img, blur_amount),
        Commands::Brighten { brighten_amount } => img = brighten(img, brighten_amount),
        Commands::Crop {
            x,
            y,
            width,
            height,
        } => img = crop(&mut img, x, y, width, height),
        Commands::Rotate { rotate_amount } => img = rotate(img, rotate_amount),
        Commands::Invert => invert(&mut img),
        Commands::Grayscale => img = grayscale(img),
        Commands::Generate {
            red_amount,
            green_amount,
            blue_amount,
        } => img = generate(red_amount, green_amount, blue_amount),
        Commands::Fractal => img = fractal(),
    }

    // save the image
    img.save(args.outfile).expect("Failed writing OUTFILE.");
}

/// **Blur** the image by the given amount.
fn blur(img: DynamicImage, blur_amount: f32) -> DynamicImage {
    img.blur(blur_amount)
}

/// **Brighten** the image by the given amount.
fn brighten(img: DynamicImage, brighten_amount: i32) -> DynamicImage {
    img.brighten(brighten_amount)
}

/// **Crop** the image to a fixed width/height starting at the given x/y position.
fn crop(img: &mut DynamicImage, x: u32, y: u32, width: u32, height: u32) -> DynamicImage {
    img.crop(x, y, width, height)
}

/// **Rotate** the image 90 degrees left/right or flip it by rotating 180 degrees.
fn rotate(img: DynamicImage, rotate_amount: RotateAmount) -> DynamicImage {
    match rotate_amount {
        RotateAmount::Right => img.rotate90(),
        RotateAmount::Flip => img.rotate180(),
        RotateAmount::Left => img.rotate270(),
    }
}

/// **Invert** the image colours (create a negative).
fn invert(img: &mut DynamicImage) {
    img.invert();
}

/// Convert image to **Grayscale** by removing all colour.
fn grayscale(img: DynamicImage) -> DynamicImage {
    img.grayscale()
}

/// **Generate** a fun image.
fn generate(red: u8, green: u8, blue: u8) -> DynamicImage {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Generate a pretty fractal and scale by supplied colour values
        // let r =
        //     (255 - (0.333 * (x - 20) as f32) as u8) * (255 - (0.333 * (y - 20) as f32) as u8) * red;
        // let g = ((0.333 * (x - 20) as f32) as u8) * (255 - (0.333 * (y - 20) as f32) as u8) * green;
        // let b = (255 - (0.333 * (x - 20) as f32) as u8) * ((0.333 * (y - 20) as f32) as u8) * blue;

        // Generate a colour gradient scaled by supplied colour values
        let y_percent = y as f32 / height as f32;
        let x_percent = x as f32 / width as f32;

        let r = ((255
            - ((y_percent - x_percent) * 255.0) as u8
            - ((x_percent - y_percent) * 255.0) as u8) as f32
            * (red as f32 / 255.0)) as u8;
        let g = (y_percent * green as f32) as u8;
        let b = (x_percent * blue as f32) as u8;

        *pixel = image::Rgb([r, g, b]);

        // set pixel values for each pixel - simple color swatch
        // *pixel = image::Rgb([red, green, blue]);
    }

    image::DynamicImage::ImageRgb8(imgbuf)
}

// This code was adapted from https://github.com/PistonDevelopers/image
/// Generate a **fractal** image.
fn fractal() -> DynamicImage {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    image::DynamicImage::ImageRgb8(imgbuf)
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
