extern crate rscam;

use rscam::{Camera, ResolutionInfo};

fn main() {
    let camera = Camera::new("/dev/video0").unwrap();

    for wformat in camera.formats() {
        let format = wformat.unwrap();
        println!("{:?}", format);

        for resolution in camera.iter_resolutions(&format.format).unwrap() {
            for interval in camera.iter_intervals(&format.format, resolution).unwrap() {
                println!(
                    "  {}x{}\t{}fps",
                    resolution.0,
                    resolution.1,
                    interval.1 / interval.0,
                );
            }
        }
    }
}
