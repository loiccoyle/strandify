<p align="center"><img src="https://i.imgur.com/4jvon2p.png" width="1000"></p>
<p align="center"><b>A string art generation library.</b></p>

# Main Structs

## [`Pather`](crate::pather::Pather)

The `Pather` struct is responsible for computing the path between pegs and generates a [`Blueprint`](crate::blueprint::Blueprint). The pathing algorithm is configured with the [`PatherConfig`](crate::pather::PatherConfig).

## [`PatherConfig`](crate::pather::PatherConfig)

The `PatherConfig` struct contains configuration parameters for computing the string path.

## [`Blueprint`](crate::blueprint::Blueprint)

The `Blueprint` struct represents computed string path between the pegs. It contains the peg order and provides method to render it to file.

## [`Yarn`](crate::peg::Yarn)

The `Yarn` struct is used to control how to render the image, and it is also used to influence the pathing algorithm.

## [`Peg`](crate::peg::Peg)

The `Peg` struct represents a peg in the yarn pattern.

# Helpful functions

`strandify` provides a few function which could come in handy.

## Opening images

To help with handling alpha channels use [`open_img_transparency_to_white`](crate::utils::open_img_transparency_to_white).

## Peg shapes

`strandify` provides a few helpful function to help position [`Pegs`](crate::peg::Peg) in various shapes:

- [`circle`](crate::peg::shape::circle)
- [`rectangle`](crate::peg::shape::rectangle)
- [`square`](crate::peg::shape::square)
- [`line`](<crate::peg::shape::line()>)

# Usage

Provided is a snippet showcasing some basis usage.

```rust
use strandify::blueprint;
use strandify::peg;
use strandify::pather;
use strandify::utils;

let input_file="tests/input.jpg";
let output_file = "tests/output.png";

// Open the input image and convert it to grayscale
let img_rgb = utils::open_img_transparency_to_white(input_file).unwrap();
let img = image::imageops::grayscale(&img_rgb);

// Define the pegs for the pathing
let (width, height) = img_rgb.dimensions();
let min_dim = std::cmp::min(width, height);
let margin = (min_dim as f64 * 0.02).round() as u32; // 2% margin
let center = (width / 2, height / 2);

// We'll be using a circle
let pegs = peg::shape::circle(center, (min_dim - 2 * margin) / 2, 100);

// Set up the configuration for the Pather
let config = pather::PatherConfig::default();

// Generate the yarn pattern
let mut string_pather = pather::Pather::new(img, pegs, config);
let bp = string_pather.compute().unwrap();

// Save the generated blueprint
bp.render(&output_file, &peg::Yarn::default()).unwrap();
```
