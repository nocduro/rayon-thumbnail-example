extern crate glob;
extern crate image;
extern crate rayon;

use std::path::{Path, PathBuf};
use std::fs;
use std::fs::File;

use glob::{glob_with, MatchOptions};
use image::{FilterType, ImageResult};
use rayon::prelude::*;

fn main() {
    let files = find_jpg();
    println!("Found {} files to convert", files.len());
    
    for problem in parallel_resize(&files, Path::new("thumbnails"), 300) {
        println!("{:?} failed with error: {}", problem.file_path, problem.err);
    }
}

/// Find all files that have a `.jpg` extension in the current directory
fn find_jpg() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    for entry in glob_with("*.jpg", &options).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => paths.push(path),
            Err(e) => println!("Error with: {:?}", e),
        }
    }

    paths
}

/// Resize `file` to have a maximum dimension of `longest_edge` and save the resized
/// image to the `thumb_dir` folder
fn resize_jpg(file: &Path, thumb_dir: &Path, longest_edge: u32) -> ImageResult<()> {
    // create the folder if it doesn't exist
    fs::create_dir_all(thumb_dir)?;
    let img = image::open(file)?;
    let img = img.resize(longest_edge, longest_edge, FilterType::Nearest);

    let output_path = thumb_dir.join(file);

    let ref mut fout = File::create(output_path.as_path())?;
    img.save(fout, image::JPEG)?;

    Ok(())
}

#[derive(Debug)]
struct ImageProblem {
    file_path: PathBuf,
    err: image::ImageError,
}

/// Wrapper for `resize_jpg` to resize multiple images at the same time with rayon
/// Returns a vector of the files that had an error during conversion, along with the actual error
fn parallel_resize(files: &[PathBuf], thumb_dir: &Path, longest_edge: u32) -> Vec<ImageProblem> {
    files
        .par_iter()
        .map(|path| {
            resize_jpg(path, thumb_dir, longest_edge).map_err(|e| {
                ImageProblem {
                    file_path: path.clone(),
                    err: e,
                }
            })
        })
        .filter_map(|x| x.err())
        .collect()
}
