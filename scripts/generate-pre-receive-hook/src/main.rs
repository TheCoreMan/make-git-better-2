use log;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use simple_logger;
use std::fs;
use std::io::Write;
use structopt::StructOpt;
use tinytemplate::TinyTemplate;
use toml;

#[derive(Debug, StructOpt)]
#[structopt(about = "A script to generate the master pre-receive hook file.")]
struct Cli {
    #[structopt(parse(from_os_str), help = "Path to game config file to read")]
    game_config_path: std::path::PathBuf,

    #[structopt(parse(from_os_str), help = "Path to template file to read")]
    template_path: std::path::PathBuf,

    #[structopt(
        parse(from_os_str),
        default_value = "output/pre-receive",
        help = "Path to output file (creates if doesn't exist)"
    )]
    output_path: std::path::PathBuf,

    #[structopt(
        short = "v",
        long = "verbose",
        help = "Show more information about the actions taken"
    )]
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

    if args.verbose {
        simple_logger::init_with_level(log::Level::Debug).unwrap();
    } else {
        simple_logger::init_with_level(log::Level::Info).unwrap();
    };

    info!("Reading script from {:?}", args.game_config_path);
    let game_config_file_contents = fs::read_to_string(args.game_config_path).unwrap();

    let game_config: GameConfig = toml::from_str(&game_config_file_contents).unwrap();

    debug!("########## GAME CONFIG STRUCT ##########");
    debug!("{:?}\n", game_config);

    info!("Reading template from {:?}", args.template_path);
    let template_file_contents = fs::read_to_string(args.template_path).unwrap();

    let mut tt = TinyTemplate::new();
    let template_name = "switch_case";
    tt.add_template(template_name, &template_file_contents)
        .unwrap();
    let rendered = tt.render(template_name, &game_config).unwrap();

    debug!("########## RENDERED TEMPLATE ##########");
    debug!("{}\n", rendered);

    let mut output_dir = args.output_path.clone();
    output_dir.pop();
    fs::create_dir_all(&output_dir).expect("Failed to create parent dir");
    let mut output_file = fs::File::create(&args.output_path).expect("Couldn't create file!");
    output_file.write_all(&rendered.as_bytes()).unwrap();

    info!("Wrote rendered file to {:?}", args.output_path);
}
