use log::{debug, info};
use simple_logger;
use std::fs;
use std::io::Write;
use structopt::StructOpt;
use tinytemplate::TinyTemplate;
use toml;

use common::GameConfig;

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

fn replace_flags_with_branch_names(game_config: &mut GameConfig) {
    let levels_info = game_config.levels.clone();

    for mut level in &mut game_config.levels {
        let mut new_flags = Vec::new();
        for flag in &level.flags {
            debug!("level {} flag {}", level.title, flag);
            let mut levels_iterator = levels_info.iter();
            let found = levels_iterator.find(|&x| &x.title == flag);
            match found {
                Some(x) => {
                    debug!("replacing {} with {}", flag, x.branch);
                    new_flags.push(String::from(&x.branch));
                }
                None => {
                    debug!("flag {} is final", flag);
                    new_flags.push(flag.to_string());
                }
            }
        }
        level.flags = new_flags;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{GameConfig, Level};

    #[test]
    fn test_replace_flags_with_branch_names() {
        let first_level = Level {
            title: "a".to_string(),
            branch: "a".to_string(),
            solution_checker: "a".to_string(),
            flags: vec!["second_level_title".to_string()],
        };
        let second_level = Level {
            title: "second_level_title".to_string(),
            branch: "second_level_branch".to_string(),
            solution_checker: "b".to_string(),
            flags: vec!["c".to_string()],
        };
        let mut game_conf = GameConfig {
            levels: vec![first_level, second_level],
        };
        replace_flags_with_branch_names(&mut game_conf);
        assert_eq!(
            game_conf.levels[0].flags[0],
            "second_level_branch".to_string()
        );
    }
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

    let mut game_config: GameConfig = toml::from_str(&game_config_file_contents).unwrap();
    debug!("Game config before editing: {:?}\n", game_config);

    replace_flags_with_branch_names(&mut game_config);

    debug!("Game config after editing: {:?}\n", game_config);

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
