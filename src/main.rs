mod frames;
mod obj;
use clap::{App, Arg, SubCommand};

fn main() {
    let app = App::new("Khan Acadamey Bad Apple Preparer")
        .about("for the memes")
        .version("1.0")
        .author("realSaddy");

    let app = app.subcommand(
        SubCommand::with_name("svgify").about("Convert directory of images to directory of SVGs"),
    );

    let app = app.subcommand(
        SubCommand::with_name("khanify")
            .about("Convert directory of SVGs to PJS array")
            .arg(
                Arg::with_name("input")
                    .short("i")
                    .required(true)
                    .help("Input directory with SVGs")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("output")
                    .short("o")
                    .required(true)
                    .help("File to output to")
                    .takes_value(true),
            ),
    );

    let matches = app.get_matches();

    if let Some(matches) = matches.subcommand_matches("svgify") {
        println!("Starting to Process Frames...");
        frames::process_frames();
    }
    if let Some(matches) = matches.subcommand_matches("khanify") {
        println!("Starting Khanify...");
        obj::khanify(
            matches.value_of("input").unwrap(),
            matches.value_of("output").unwrap(),
        );
    }
}
