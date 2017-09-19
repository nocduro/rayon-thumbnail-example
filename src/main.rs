#[macro_use]
extern crate error_chain;
extern crate glob;
extern crate image;
extern crate rayon;

use std::path::Path;
use std::fs::{create_dir_all, File};

use glob::{glob_with, MatchOptions};
use image::{FilterType, ImageError};
use rayon::prelude::*;

error_chain! {
    foreign_links {
        Image(ImageError);
        Io(std::io::Error);
        Glob(glob::PatternError);
    }
}

fn run() -> Result<()> {
    // find all files in current directory that have a .jpg extension
    // use the default MatchOptions so the search is case insensitive
    let options: MatchOptions = Default::default();
    let files: Vec<_> = glob_with("*.jpg", &options)?
        .filter_map(|x| x.ok())
        .collect();

    let thumb_dir = Path::new("thumbnails");
    create_dir_all(thumb_dir)?;

    println!("Saving {} thumbnails into {:?}", files.len(), thumb_dir);

    let image_failures: Vec<String> = files
        .par_iter()
        .map(|path| -> std::result::Result<(), String> {
            match make_thumbnail(path, thumb_dir, 300) {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("{:?} failed: {}", path, e)),
            }
        })
        .filter_map(|x| x.err())
        .collect();

    for failure in image_failures {
        println!("{}", failure);
    }

    println!("Done");
    Ok(())
}

/// Resize `original` to have a maximum dimension of `longest_edge` and save the resized
/// image to the `thumb_dir` folder
fn make_thumbnail(original: &Path, thumb_dir: &Path, longest_edge: u32) -> Result<()> {
    let output_path = thumb_dir.join(original);
    let fout = &mut File::create(output_path)?;

    image::open(original)?
        .resize(longest_edge, longest_edge, FilterType::Nearest)
        .save(fout, image::JPEG)?;
    Ok(())
}

quick_main!(run);
