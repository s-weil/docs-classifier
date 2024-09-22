use std::collections::VecDeque;
use std::error::Error;
use std::path::PathBuf;

use ocrs::{ImageSource, OcrEngine, OcrEngineParams};
use rten::Model;
#[allow(unused)]
use rten_tensor::prelude::*;

#[derive(Debug)]
struct Args {
    image: String,
}

fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    let mut values = VecDeque::new();
    let mut parser = lexopt::Parser::from_env();

    while let Some(arg) = parser.next()? {
        match arg {
            Value(val) => values.push_back(val.string()?),
            Long("help") => {
                println!(
                    "Usage: {bin_name} <image>",
                    bin_name = parser.bin_name().unwrap_or("hello_ocrs")
                );
                std::process::exit(0);
            }
            _ => return Err(arg.unexpected()),
        }
    }

    let image = values.pop_front().ok_or("missing `image` arg")?;

    Ok(Args { image })
}

/// Given a file path relative to the crate root, return the absolute path.
fn file_path(path: &str) -> PathBuf {
    let mut abs_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    abs_path.push(path);
    abs_path
}

fn model_path(path: &str) -> PathBuf {
    // let mut abs_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut models_dir = PathBuf::new();
    models_dir.push("/home/steffen/Code/python/docs-classifier/rust/models");
    models_dir.push(path);
    models_dir
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = parse_args()?;

    println!("Running with {:?}", args);
    let image_path = file_path(&args.image);
    println!("Sourcing image from {:?}", image_path);

    // let image = "/home/steffen/Code/python/docs-classifier/rust/resources/rust-book.jpg";
    let image = "/home/steffen/Code/python/docs-classifier/rust/resources/angebot_0.jpg";
    let args = Args {
        image: image.to_string(),
    };

    // Use the `download-models.sh` script to download the models.
    let detection_model_path = model_path("text-detection.rten");
    let rec_model_path = model_path("text-recognition.rten");

    let detection_model = Model::load_file(detection_model_path)?;
    let recognition_model = Model::load_file(rec_model_path)?;

    let engine = OcrEngine::new(OcrEngineParams {
        detection_model: Some(detection_model),
        recognition_model: Some(recognition_model),
        ..Default::default()
    })?;

    // Read image using image-rs library, and convert to RGB if not already
    // in that format.
    let img = image::open(&args.image).map(|image| image.into_rgb8())?;

    // Apply standard image pre-processing expected by this library (convert
    // to greyscale, map range to [-0.5, 0.5]).
    let img_source = ImageSource::from_bytes(img.as_raw(), img.dimensions())?;
    let ocr_input = engine.prepare_input(img_source)?;

    // Detect and recognize text. If you only need the text and don't need any
    // layout information, you can also use `engine.get_text(&ocr_input)`,
    // which returns all the text in an image as a single string.

    // Get oriented bounding boxes of text words in input image.
    let word_rects = engine.detect_words(&ocr_input)?;

    // Group words into lines. Each line is represented by a list of word
    // bounding boxes.
    let line_rects = engine.find_text_lines(&ocr_input, &word_rects);

    // Recognize the characters in each line.
    let line_texts = engine.recognize_text(&ocr_input, &line_rects)?;

    for line in line_texts
        .iter()
        .flatten()
        // Filter likely spurious detections. With future model improvements
        // this should become unnecessary.
        .filter(|l| l.to_string().len() > 1)
    {
        println!("{}", line);
    }

    Ok(())
}
