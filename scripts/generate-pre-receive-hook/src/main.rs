use std::fs;
use structopt::StructOpt;

// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, StructOpt)]
struct Cli {
    // The path to the file to read
    #[structopt(parse(from_os_str))]
    game_config_path: std::path::PathBuf,
}

struct Level {
    title: String,
    branch: String,
    solution_checke: String,
    flags: Vec<String>,
}

struct GameConfig {
    levels: Vec<Level>,
}

fn main() {
    let args = Cli::from_args();
    println!("Loading script from {:?}", args);

    let game_config_file_contents =
        fs::read_to_string(args.game_config_path).expect("Couldn't read the config file!");

    println!("{}", game_config_file_contents);
}
