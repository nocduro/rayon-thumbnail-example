
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

    for entry in glob("*.jpg").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                if let Some(parent) = path.parent() {
                    if parent.ends_with("thumbs") {
                        // skip already generated thumbnails
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

fn resize_jpg(file: &Path, thumb_dir: &Path, longest_edge: u32) -> ImageResult<()> {
    fs::create_dir_all(thumb_dir)?;
    let img = image::open(file)?;
    let img = img.resize(longest_edge, longest_edge, FilterType::Nearest);

    let output_path = thumb_dir.join(file);

    let ref mut fout = File::create(output_path.as_path())?;
    let _ = img.save(fout, image::JPEG)?;

    Ok(())
}
/*
fn sequential(files: &[PathBuf], longest_edge: u32) -> ImageResult<()> {
    fs::create_dir_all("thumbs")?;

    for jpg in files.iter() {
        resize_jpg(jpg, longest_edge)?;
    }
    Ok(())
}*/

#[derive(Debug)]
struct ImageProblem {
    file_path: PathBuf,
    error: image::ImageError,
}

fn parallel(files: &[PathBuf], thumb_dir: &Path, longest_edge: u32) -> Vec<ImageProblem> {
    //fs::create_dir_all("thumbs").map_err(|e| ImageProblem{file_path: "thumbs", error: e});
    files
        .par_iter()
        .map(|path| {
            resize_jpg(path, thumb_dir, longest_edge).map_err(|e| {
                ImageProblem {
                    file_path: path.clone(),
                    error: e,
                }
            })
        })
        .filter_map(|x| x.err())
        .collect()
}




fn main() {
    let files = find_jpg();
    println!("Found {} files to convert", files.len());

    for problem in parallel(&files, Path::new("thumbnails"), 300) {
        println!("{:?} failed with error: {}", problem.file_path, problem.error);
    }
}
