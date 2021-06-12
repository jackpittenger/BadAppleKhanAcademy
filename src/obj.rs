use std::{
    fs::{self, File},
    io::{BufReader, Read, Write},
    path::PathBuf,
};

use glob::{glob_with, MatchOptions};
// Steps:
// 1. Take in a directory of SVGs
// 2. Sort SVGs by #
// 3. Load SVG and extract D
//  4. For each D, extract all bezier curves
//  5. Add each bezier curve to "array" text
// 6. Append these all to a single "array"
// 7. Save the file

pub fn khanify(input: &str, output: &str) {
    println!("{} {}", input, output);
    let options: MatchOptions = Default::default();
    let mut paths: Vec<_> = glob_with([input, "/*"].join("").as_str(), options)
        .unwrap()
        .map(|x| x.ok())
        .collect();
    paths.sort();
    let mut file = File::create(output).ok().unwrap();

    let new_line = "\n".as_bytes();
    let open_bracket = "[".as_bytes();
    let close_bracket = "]".as_bytes();
    let comma = ",".as_bytes();
    let semicolon = ";".as_bytes();

    file.write(open_bracket);

    paths.into_iter().for_each(|path| {
        let dir: PathBuf = path.unwrap();

        file.write(open_bracket);
        println!("{:?}", dir);

        // ===
        // Handle svg
        // ===
        let svg = File::open(dir).unwrap();
        let mut buf_reader = BufReader::new(svg);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents);

        // Remove first two and last line (lines with no path)
        let split = contents.split("\n");
        let mut vec = split.collect::<Vec<&str>>();
        vec.remove(0);
        vec.remove(0);
        vec.pop();
        vec.pop();

        vec.iter().for_each(|contents| {
            // Remove first part of SVG
            let split = contents.split("M0 0 C");
            let vec = split.collect::<Vec<&str>>();

            // Stop if empty
            if vec.len() < 2 {
                return;
            }
            let contents = vec[1].to_string();

            // Remove last part of SVG
            let split = contents.split(" Z");
            let vec = split.collect::<Vec<&str>>();

            let contents = vec[0].to_string();
            let transform = vec[vec.len()-1].to_string();

            // Get transform
            let split = transform.split("translate(");
            let vec = split.collect::<Vec<&str>>();
            let transform = vec[1].to_string();
            // remove last part
            let split = transform.split(")");
            let vec = split.collect::<Vec<&str>>();
            let transform = vec[0].to_string();
            
            // Separate curves
            let split = contents.split(" C");
            let vec = split.collect::<Vec<&str>>();
            
            vec.iter().for_each(|s| {
                file.write(open_bracket);
                let s_split = s.split(" ");
                let v = s_split.collect::<Vec<&str>>();
                file.write(v.join(",").as_bytes());
                file.write(comma);
                file.write(transform.as_bytes());
                file.write(close_bracket);
                file.write(comma);
            });
      
            // at this point it should be usize,usize
            // So what this means is we have [ [curves]..[curves],tx,ty] ]
        });
        
        
        file.write(close_bracket);
        file.write(comma);
        file.write(new_line);
    });

    file.write(close_bracket);
    file.write(semicolon);
}
