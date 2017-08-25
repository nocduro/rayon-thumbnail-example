
extern crate glob;
extern crate image;
extern crate rayon;

use glob::glob;
use image::{ImageResult, FilterType};
use std::path::{Path, PathBuf};
use std::fs;
use std::fs::File;
use rayon::prelude::*;

fn find_jpg() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    for entry in glob("**/*.jpg").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                if let Some(parent) = path.parent() {
                    // skip already generated thumbnails
                    if parent.ends_with("thumbs") {
                        continue;
                    }
                }
                paths.push(path);
            }
            Err(e) => println!("Error with: {:?}", e),
        }
    }

    paths
}

fn resize_jpg(file: &Path, longest_edge: u32) -> ImageResult<()> {
    let img = image::open(file)?;
    let img = img.resize(longest_edge, longest_edge, FilterType::Lanczos3);

    let output_path = Path::new("thumbs").join(file);

    let ref mut fout = File::create(output_path.as_path())?;
    let _ = img.save(fout, image::JPEG)?;

    Ok(())
}

fn sequential(files: &[PathBuf], longest_edge: u32) -> ImageResult<()> {
    fs::create_dir_all("thumbs").expect("Couldn't make thumbnail directory");

    for jpg in files.iter() {
        resize_jpg(jpg, longest_edge)?;
    }
    Ok(())
}

fn parallel(files: &[PathBuf], longest_edge: u32) -> ImageResult<()> {
    fs::create_dir_all("thumbs")?;

    files
        .par_iter()
        .for_each(|jpg| { resize_jpg(jpg, longest_edge).unwrap(); });

    Ok(())
}

fn main() {
    let files = find_jpg();

    match sequential(&files, 300) {
        Ok(()) => println!("{} thumbnails saved", files.len()),
        Err(e) => println!("Failed to create all thumbnails: {:?}", e),
    }

}
