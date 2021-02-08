use rand::seq::IteratorRandom;
use std::fs;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "A script to get random branch name")]
struct Cli {
    #[structopt(
        parse(from_os_str),
        default_value = "resources/words_alpha.txt",
        help = "Path to a file with english words separated by newline"
    )]
    words_path: std::path::PathBuf,
}

fn get_random_branch_name_from_word_list(words: &Vec<String>) -> String {
    let random_words: Vec<&String> = words.iter().choose_multiple(&mut rand::thread_rng(), 3);
    let cloned: Vec<String> = random_words.into_iter().map(|x| x.clone()).collect();
    cloned.join("-")
}

fn main() {
    let args = Cli::from_args();

    // First, load all the necessary data files and templates.
    let all_words_file_contents = fs::read_to_string(args.words_path).unwrap();
    let all_words: Vec<String> = all_words_file_contents.lines().map(|l| l.to_string()).collect();

    let level_branch_name: String = get_random_branch_name_from_word_list(&all_words);
    println!("{}", level_branch_name);
}
