extern crate image;
extern crate lapp;

use image::GenericImageView;

fn main() {
    // Parsing Arguments
    /*
    let args = lapp::parse_args("
        Solves 


        Prints out first n lines of a file
        -n, --lines (default 10) number of lines
        -v, --verbose
        <file> (string) input file name
        <file> (string) output file name
    ");
    
	let _n = args.get_integer("lines");
	let _verbose = args.get_bool("verbose");
	let _file = args.get_string("file");
	// your magic goes here*/

    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open("Mazes/M10x10.bmp").unwrap();
    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());
    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());
    //println!("{:?}", img);
}

struct Maze {
    start_position: (u32, u32),
    end_position: (u32, u32),


}

trait Algorithm {
    fn 
}

struct A_Star_Algorithm {

}