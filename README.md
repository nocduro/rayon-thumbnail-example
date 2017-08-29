# rayon-image-resizer

## Overview
Uses the `glob`, `image`, and `rayon` crates to resize all of the .jpg in the current directory
into a `thumbnails` folder.

## Usage
run with: `cargo run --release` with some .jpg images in the top level directory of the repository.

## Results
Test was run against 196 images from an 18MP dslr (~1GB of images). 

Used the `Nearest` resize algorithm from the `image` crate, resizing to a 300px thumbnail.

Results from i5 4670k (4 cores/threads):

* No rayon: 63.837 seconds  ~50% cpu usage 
* With rayon: 33.094 seconds  ~97% cpu usage