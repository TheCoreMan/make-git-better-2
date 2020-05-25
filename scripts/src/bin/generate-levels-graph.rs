use log::{debug, info};
use petgraph::graph::NodeIndex;
use petgraph::{Directed, Graph};
use std::fs;
use structopt::StructOpt;

use common::{GameConfig, Level};

type LevelsGraph<'a> = Graph<&'a Level, (), Directed>;
type Nodes<'a> = Vec<&'a NodeIndex>;

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

/// Recursive function that populates the game graph.NodeIndex
/// 
/// If receives a graph initialized with the first level as a root node.
fn add_level_nodes_to_graph<'a, 'gl>(
    current_level: Level,
    current_node: &'gl NodeIndex,
    graph: &'gl mut LevelsGraph,
    game_config: &'a GameConfig,
) -> Nodes<'gl> {
    let mut new_nodes = Nodes::new();
    if current_level.flags.len() == 0 {
        return new_nodes;
    };

    for flag in current_level.flags {
        debug!("level {} flag {}", current_level.title, flag);
        let levels_iterator = game_config.levels.iter();
        let found = levels_iterator.find(|x| x.title == flag);
        match found {
            Some(x) => {
                debug!("The flag does point to another level, {}. Adding level as node to graph", x.title);
                // What's the issue here? 
                let new_node = graph.add_node(x);
                new_nodes.push(&new_node);
                debug!("Adding edge from {} to {}", current_level.title, x.title);
                graph.add_edge(*current_node, new_node, ());
                debug!("Recursive calling add nodes on {}", x.title);
                let sub_nodes = add_level_nodes_to_graph(
                    *x, 
                    &new_node,
                    graph, 
                    &game_config);
                // new_nodes.append(sub_nodes);
            }
            None => {
                debug!("The flag doesn't point to another level - no need to recurse");
            }
        }
    };
    return new_nodes;
}

fn create_graph_from_game_config(game_config: &GameConfig) -> LevelsGraph {
    let mut levels_graph = LevelsGraph::new();

    let first_level = game_config.levels[0];
    let tree_root = levels_graph.add_node(&first_level);
    let nodes = add_level_nodes_to_graph(
        first_level,
        &tree_root,
        &mut levels_graph, 
        &game_config
    );

    return levels_graph;
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

    let levels_graph = create_graph_from_game_config(&game_config);

    println!("{:?}", levels_graph);
}
