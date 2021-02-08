use log::{debug, info};
use petgraph::dot::{Config, Dot};
use petgraph::graph::NodeIndex;
use petgraph::{Directed, Graph};
use serde::Serialize;
use std::fs;
use std::io::Write;
use structopt::StructOpt;
use tinytemplate::TinyTemplate;

use common::{GameConfig, Level};

type LevelsGraph = Graph<Level, i32, Directed>;

#[derive(Debug, StructOpt)]
#[structopt(about = "A script to generate a levels graph from a game config.")]
struct Cli {
    #[structopt(parse(from_os_str), help = "Path to game config file to read")]
    game_config_path: std::path::PathBuf,

    #[structopt(parse(from_os_str), help = "Path to the graph template file to read")]
    template_path: std::path::PathBuf,

    #[structopt(
        parse(from_os_str),
        default_value = "output/levelgraph.html",
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

/// Recursive function that populates the game graph
///
/// If receives a graph initialized with the first level as a root node.
fn add_level_nodes_to_graph<'a>(
    current_level: Level,
    current_node: &'a NodeIndex,
    levels_graph: &'a mut LevelsGraph,
    game_config: &'a GameConfig,
) {
    if current_level.flags.len() == 0 {
        return;
    };

    for flag in current_level.flags {
        debug!("level {} flag {}", current_level.title, flag);
        let mut levels_iterator = game_config.levels.iter();
        let found = levels_iterator.find(|x| x.title == flag);
        match found {
            Some(x) => {
                debug!(
                    "The flag does point to another level, {}. Adding level as node to graph",
                    x.title
                );
                let new_node = levels_graph.add_node(x.clone());
                debug!("Adding edge from {} to {}", current_level.title, x.title);
                levels_graph.add_edge(*current_node, new_node, 0);
                debug!("Recursive calling add nodes on {}", x.title);
                add_level_nodes_to_graph(x.clone(), &new_node, levels_graph, &game_config);
            }
            None => {
                debug!("The flag doesn't point to another level - no need to recurse");
            }
        }
    }
}

fn create_graph_from_game_config(game_config: &GameConfig) -> LevelsGraph {
    let mut levels_graph = LevelsGraph::new();

    let first_level = game_config.levels[0].clone();
    let tree_root = levels_graph.add_node(first_level.clone());
    add_level_nodes_to_graph(first_level, &tree_root, &mut levels_graph, &game_config);

    levels_graph
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::algo::is_cyclic_directed;

    #[test]
    fn test_create_graph_from_game_config() {
        let first_level = Level {
            title: String::from("first"),
            branch: String::from("first"),
            solution_checker: String::from("first"),
            flags: vec!["second".to_string()],
        };
        let second_level = Level {
            title: String::from("second"),
            branch: String::from("sec"),
            solution_checker: String::from("sec"),
            flags: vec!["another".to_string(), "asdf".to_string()],
        };

        let game_conf = GameConfig {
            levels: vec![first_level, second_level],
        };
        let graph = create_graph_from_game_config(&game_conf);

        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
        assert!(graph.is_directed());
        assert!(!is_cyclic_directed(&graph));
    }
}

#[derive(Serialize)]
struct Context {
    levels_graph_as_dot: String,
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

    let levels_graph_as_dot = Dot::with_config(&levels_graph, &[Config::EdgeNoLabel]);
    let context = Context {
        levels_graph_as_dot: format!("{}", levels_graph_as_dot),
    };

    debug!("Generated graph:\n{:?}", levels_graph_as_dot);

    info!("Reading template from {:?}", args.template_path);
    let template_file_contents = fs::read_to_string(args.template_path).unwrap();

    let mut tt = TinyTemplate::new();
    let template_name = "levels_graph";
    tt.add_template(template_name, &template_file_contents)
        .unwrap();
    let rendered = tt.render(template_name, &context).unwrap();

    debug!("########## RENDERED TEMPLATE ##########");
    debug!("{}\n", rendered);

    let mut output_dir = args.output_path.clone();
    output_dir.pop();
    fs::create_dir_all(&output_dir).expect("Failed to create parent dir");
    let mut output_file = fs::File::create(&args.output_path).expect("Couldn't create file!");
    output_file.write_all(&rendered.as_bytes()).unwrap();

    info!("Wrote rendered file to {:?}", args.output_path);
}
