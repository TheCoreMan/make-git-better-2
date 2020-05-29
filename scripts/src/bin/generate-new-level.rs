use rand::seq::IteratorRandom;
use std::fs;
use structopt::StructOpt;
use text_io::read;
use log::{debug, info, warn};
use tinytemplate::TinyTemplate;

use common::{GameConfig, Level};

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
        help = "Path to a file with english words separated by newline"
    )]
    words_path: std::path::PathBuf,

    #[structopt(
        parse(from_os_str),
        default_value = "../levels/",
        help = "Levels directory"
    )]
    levels_directory: std::path::PathBuf,

    #[structopt(
        short = "v",
        long = "verbose",
        help = "Show more information about the actions taken"
    )]
    verbose: bool,
}

fn get_random_branch_name(words: &Vec<String>) -> String {
    let random_words: Vec<&String> = words.iter().choose_multiple(&mut rand::thread_rng(), 3);
    let cloned: Vec<String> = random_words.into_iter().map(|x| x.clone()).collect();
    cloned.join("-")
}

fn get_all_titles_from_config(game_config: &GameConfig) -> Vec<String> {
    let mut all_titles: Vec<String> = Vec::new();
    
    game_config.levels.iter().for_each(|x| all_titles.push(x.title.clone()));

    all_titles
}

fn main() {
    let args = Cli::from_args();

    if args.verbose {
        println!("verbose");
        simple_logger::init_with_level(log::Level::Debug).unwrap();
    } else {
        simple_logger::init_with_level(log::Level::Info).unwrap();
    };

    // First, load all the necessary data files and templates.
    debug!("Loading words file from {}", args.words_path.display());
    let all_words_file_contents = fs::read_to_string(args.words_path).unwrap();
    let all_words: Vec<String> = all_words_file_contents.lines().map(|l| l.to_string()).collect();

    debug!("Loading game config from {}", args.game_config_path.display());
    let game_config_file_contents = fs::read_to_string(&args.game_config_path).unwrap();
    let mut game_config: GameConfig = toml::from_str(&game_config_file_contents).unwrap();

    println!("What is the new level title?");
    print!("> ");
    let level_title: String = read!();

    println!("Choose a branch name. Enter y to approve anything else to generate new branch name");
    let mut approved: bool = false;
    let mut level_branch_name: String = get_random_branch_name(&all_words);
    while !approved {
        level_branch_name = get_random_branch_name(&all_words);
        println!("Branch name: {}", level_branch_name);
        let user_approved: String = read!();
        approved = user_approved.to_lowercase() == "y".to_string()
    }

    let level_solution_checker: String = format!("hooks/checkers/{}.sh", level_title);
    println!("Checker file path: {}", level_solution_checker);

    let all_titles = get_all_titles_from_config(&game_config);
    println!("All (current) existing titles: {:?}", all_titles);
    println!("What are the flags the player should get for solving {}? Separate flags with `,` (commas).", level_title);
    print!("> ");
    let level_flags_string: String = read!();
    let level_flags: Vec<String> = level_flags_string.split(',').map(|x| x.to_string()).collect();
    println!("{:?}", level_flags);
    for level_flag in &level_flags {
        if all_titles.iter().find(|&x| x == level_flag).is_none() {
           warn!("{} doesn't exist in configuration but supplied as flag", level_flag);
        }
    }

    let new_level = Level {
        title: level_title,
        branch: level_branch_name,
        solution_checker: level_solution_checker,
        flags: level_flags,
    };

    info!("New level about to be generated: {:?}", new_level);

    let checker_info = LevelTemplateInfo {
        template_path: args.checker_template_path,
        output_path: args.levels_directory.join("checkers").join(format!("{}.{}", &new_level.title, "sh")),
    };
    let test_info = LevelTemplateInfo {
        template_path: args.test_template_path,
        output_path: args.levels_directory.join("tests").join(format!("{}.{}", &new_level.title, "sh")),
    };
    let page_info = LevelTemplateInfo {
        template_path: args.page_template_path,
        output_path: args.levels_directory.join("pages").join(format!("{}.{}", &new_level.title, "md")),
    };
    let templates_infos = vec![&checker_info, &test_info, &page_info];
    create_new_level_files(&new_level, &templates_infos);
    add_new_level_to_config(&new_level, &mut game_config, &args.game_config_path);
}

struct LevelTemplateInfo {
    template_path: std::path::PathBuf,
    output_path: std::path::PathBuf,
}

fn create_new_level_files(
    new_level: &Level, 
    templates_infos: &Vec<&LevelTemplateInfo>) 
{
    for template_info in templates_infos {
        info!("Reading template from {:?}", template_info.template_path);
        let template_file_contents = fs::read_to_string(&template_info.template_path).unwrap();

        let mut tt = TinyTemplate::new();
        let template_name = template_info.template_path.file_name().unwrap().to_str().unwrap();
        tt.add_template(template_name, &template_file_contents)
            .unwrap();
        let rendered = tt.render(template_name, &new_level).unwrap();
    
        debug!("########## RENDERED TEMPLATE ##########");
        debug!("{}\n", rendered);
    
        info!("Writing to {}", template_info.output_path.display());
        fs::write(&template_info.output_path, &rendered.as_bytes()).unwrap();
    }
}

fn add_new_level_to_config(
    new_level: &Level, 
    game_config: &mut GameConfig,
    game_config_path: &std::path::PathBuf) 
{
    game_config.levels.push(new_level.clone());
    let new_conf_as_toml = toml::to_string_pretty(&game_config).unwrap();
    debug!("New config file: {}", new_conf_as_toml);
    fs::write(game_config_path, new_conf_as_toml).unwrap();
}
