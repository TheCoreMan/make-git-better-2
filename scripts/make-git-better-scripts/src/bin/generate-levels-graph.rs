use structopt::StructOpt;
use std::fs;
use log::{debug, info};
use petgraph::{Graph, Directed};
use petgraph::graph::NodeIndex;

use common::{GameConfig, Level};

pub type LevelsGraph = Graph<&'static Level, &'static Level, Directed>;
pub type Nodes = Vec::<&'static NodeIndex>;

#[derive(Debug, StructOpt)]
#[structopt(about = "A script to generate a levels graph from a game config.")]
struct Cli {
    #[structopt(parse(from_os_str), help = "Path to game config file to read")]
    game_config_path: std::path::PathBuf,

    #[structopt(
        short = "v",
        long = "verbose",
        help = "Show more information about the actions taken"
    )]
    verbose: bool,
}

fn add_level_nodes_to_graph(current_level: Level, graph: &Graph<&Level, &Level>, game_config: &GameConfig) -> Result<Nodes, &str> {
    let mut new_nodes = Vec::<&NodeIndex>::new();
    if current_level.flags.len() == 0 {
        Ok(new_nodes)
    }
    for flag in current_level.flags {
        
    }
}

fn create_graph_from_game_config(game_config: &GameConfig) -> Result<Graph<&Level, &Level>, &str> {
    let mut levels_graph = Graph::<&Level, &Level>::new();


    for level in &game_config.levels {
        //let level_node = levels_graph.add_node(level);

    }

    Ok(levels_graph)
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

}