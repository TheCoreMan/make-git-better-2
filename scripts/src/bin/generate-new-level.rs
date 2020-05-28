use rand::seq::SliceRandom;
use std::fs;
use structopt::StructOpt;
use text_io::read;

use common::{GameConfig};

#[derive(Debug, StructOpt)]
#[structopt(about = "A script to generate a new level.")]
struct Cli {
    #[structopt(parse(from_os_str), help = "Path to game config file to read")]
    game_config_path: std::path::PathBuf,

    #[structopt(
        parse(from_os_str),
        default_value = "templates/level_checker.tmpl",
        help = "Path to the chekcer template file"
    )]
    checker_template_path: std::path::PathBuf,

    #[structopt(
        parse(from_os_str),
        default_value = "templates/level_test.tmpl",
        help = "Path to the test template file"
    )]
    test_template_path: std::path::PathBuf,

    #[structopt(
        parse(from_os_str),
        default_value = "templates/level_page.tmpl",
        help = "Path to the page template file"
    )]
    page_template_path: std::path::PathBuf,

    #[structopt(
        parse(from_os_str),
        default_value = "resources/words_alpha.txt",
        help = "Path to works json File"
    )]
    words_path: std::path::PathBuf,

    #[structopt(
        short = "v",
        long = "verbose",
        help = "Show more information about the actions taken"
    )]
    verbose: bool,
}

fn get_random_branch_name(words: &Vec<String>) -> String {
    let random_words: Vec<&String> = words.choose_multiple(&mut rand::thread_rng(), 3).collect();
    random_words.join("-")
}

fn main() {
    let args = Cli::from_args();

    if args.verbose {
        simple_logger::init_with_level(log::Level::Debug).unwrap();
    } else {
        simple_logger::init_with_level(log::Level::Info).unwrap();
    };

    println!("What is the new level title?");
    let level_title: String = read!();

    println!("Choose a branch name. Enter y to approve anything else to generate new branch name");
    let approved = false;
    let all_words_file_contents = fs::read_to_string(args.words_path).unwrap();
    let all_words = all_words_file_contents.lines().map(|l| l.to_string()).collect();
    let branch_name: String;
    while !approved {
        branch_name = get_random_branch_name(&all_words);
        println!("Branch name: {}", branch_name);

    }

    let game_config_file_contents = fs::read_to_string(args.game_config_path).unwrap();
    let game_config: GameConfig = toml::from_str(&game_config_file_contents).unwrap();
}
