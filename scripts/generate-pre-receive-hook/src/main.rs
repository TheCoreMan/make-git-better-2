use serde::{Deserialize, Serialize};
use std::io::Write;
use std::fs;
use structopt::StructOpt;
use tinytemplate::TinyTemplate;
use toml;

// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, StructOpt)]
#[structopt(about = "A script to generate the master pre-receive hook file.")]
struct Cli {
    #[structopt(parse(from_os_str), help = "Path to game config file to read")]
    game_config_path: std::path::PathBuf,

    #[structopt(parse(from_os_str), help = "Path to template file to read")]
    template_path: std::path::PathBuf,

    #[structopt(parse(from_os_str), help = "Path to output file (creates if doesn't exist)")]
    output_path: std::path::PathBuf,

    #[structopt(short = "v", long = "verbose", help = "Show more information about the actions taken")]
    verbose: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct Level {
    title: String,
    branch: String,
    solution_checker: String,
    flags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct GameConfig {
    levels: Vec<Level>,
}

fn main() {
    let args = Cli::from_args();

    println!("Reading script from {:?}", args.game_config_path);
    let game_config_file_contents =
        fs::read_to_string(args.game_config_path).unwrap();

    let game_config: GameConfig =
        toml::from_str(&game_config_file_contents).unwrap();

    if args.verbose {
        println!("########## GAME CONFIG STRUCT ##########");
        println!("{:?}\n", game_config);
    }

    println!("Reading template from {:?}", args.template_path);
    let template_file_contents =
        fs::read_to_string(args.template_path).unwrap();

    let mut tt = TinyTemplate::new();
    let template_name = "switch_case";
    tt.add_template(template_name, &template_file_contents).unwrap();
    let rendered = tt.render(template_name, &game_config).unwrap();

    if args.verbose {
        println!("########## RENDERED TEMPLATE ##########");
        println!("{}\n", rendered);
    }

    let mut output_file = fs::File::create(&args.output_path).unwrap();
    output_file.write_all(&rendered.as_bytes()).unwrap();

    println!("Wrote rendered file to {:?}", args.output_path);
}
