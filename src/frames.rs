use glob::{glob_with, MatchOptions};
use rayon::prelude::*;
use std::{path::PathBuf, sync::atomic::AtomicU16};
use visioncortex::PathSimplifyMode;
use vtracer::{convert_image_to_svg, ColorMode, Config, Hierarchical};

pub fn process_frames() {
    let options: MatchOptions = Default::default();
    let mut paths: Vec<_> = glob_with("./frames/*", options)
        .unwrap()
        .map(|x| x.ok())
        .collect();
    paths.sort();
    let c = AtomicU16::new(0);
    let length = paths.len();
    paths.into_par_iter().for_each(|path| {
        let dir: PathBuf = path.unwrap();
        let file_name = dir.file_name().unwrap().to_str().unwrap();
        let config = Config {
            input_path: dir.clone(),
            output_path: std::path::PathBuf::from(
                ["./svgs/", &file_name[0..file_name.len() - 4], ".svg"].join(""),
            ),
            color_mode: ColorMode::Binary,
            hierarchical: Hierarchical::Cutout,
            filter_speckle: 4,
            color_precision: 6,
            layer_difference: 16,
            mode: PathSimplifyMode::Spline,
            corner_threshold: 60,
            length_threshold: 4.0,
            max_iterations: 45,
            splice_threshold: 10,
        };
        let result = convert_image_to_svg(config);
        match result {
            Ok(()) => {
                let n = c.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                print!("Finished {}/{}\r", n, length);
            }
            Err(msg) => {
                panic!("Failed with {}", msg);
            }
        }
    });
    println!();
    println!("All done! :)");
}
