# Pixel Art Downsampler

A command-line tool written in Rust that detects the underlying block (pixel grid) size in a pixel art image and downsamples the image accordingly. This is useful for "fixing" pixel art images that have been scaled up by repeating pixel blocks by reducing them to their original, lower resolution form.

## Features

- **Block Size Detection:** Scans each row and column of the input image and computes the greatest common divisor (GCD) of consecutive pixel run lengths. For a perfectly scaled pixel art image, the GCD corresponds to the block size.
- **Downsampling:** Once the block size is determined, the tool downsamples the image by selecting one representative pixel per block, effectively reverting the scaling.

## How It Works

1. **GCD Calculation:**  
   The program iterates over each row and column to determine the lengths of runs of identical pixels. It uses the Euclidean algorithm to compute the GCD of these lengths, which gives the block size used to scale the original pixel art.

2. **Downsampling:**  
   With the detected block size, the image is divided into blocks of that size. The tool then creates a new image by taking the top-left pixel (or any consistent pixel) from each block, reducing the image dimensions proportionally.

## Installation

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed. Then clone this repository and build the project using Cargo:

```bash
git clone https://github.com/shellkah/pixel-art-downsampler.git
cd pixel-art-downsampler
cargo build --release
```
This will produce an executable in the target/release directory.

## Usage

Run the program from the command line with the input image and the desired output image path:

```bash
./target/release/pixel-art-downsampler <input_image> <output_image>
```
For example:
```bash
./target/release/pixel-art-downsampler input.png output.png
```

### Upon execution, the program will:
 1. Open the specified input image.
 2. Detect the block size by computing the GCD of identical pixel run lengths.
 3. Downsample the image by taking one pixel per block.
 4. Save the downsampled image to the specified output path.

The detected block size will be printed to the console, along with a confirmation message once the image is saved.

## Dependencies
This project uses the following Rust crate:

 - image: For image processing, including opening, reading, and writing images.

## License
This project is licensed under the MIT License. See the LICENSE file for details.

## Contributing
Contributions are welcome! Please open issues or submit pull requests for improvements or bug fixes.