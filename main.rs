mod player;
mod graph;

use player::Player;
use graph::Graph;
use std::error::Error;
use plotlib::page::Page;
use plotlib::repr::{Histogram, HistogramBins};
use plotlib::style::BoxStyle;
use plotlib::view::ContinuousView;

fn plot_degree_distribution(
    graph: &Graph,
    output_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let degree_distribution = graph.degree_distribution();
    let mut degrees = Vec::new();
    for (degree, count) in degree_distribution {
        for _ in 0..count {
            degrees.push(degree as f64);
        }
    }
    let h = Histogram::from_slice(&degrees, HistogramBins::Count(50))
        .style(&BoxStyle::new().fill("#334CDD"));

    let cv = ContinuousView::new()
        .add(h)
        .x_label("Degree")
        .y_label("Number of nodes");

    Page::single(&cv).save(output_file)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read CSV data
    let mut file = csv::Reader::from_path("clean_players_20.csv")?;
    let mut players: Vec<Player> = Vec::new();

    for result in file.deserialize() {
        let player: Player = result?;
        players.push(player);
    }

    // Create a custom graph
    let mut graph = Graph::new();

    // Add nodes (players) to the graph
    let player_nodes = players
        .iter()
        .map(|player| graph.add_node(player.clone()))
        .collect::<Vec<_>>();

    // Add edges
    for i in 0..player_nodes.len() {
        for j in (i + 1)..player_nodes.len() {
            let ovr_diff = (players[i].overall as i32 - players[j].overall as i32).abs();
            if ovr_diff == 0 {
                graph.add_edge(player_nodes[i], player_nodes[j]);
            }
        }
    }
    let degree_distribution = graph.degree_distribution();
    
    // Calculate the total number of nodes
    let total_nodes = graph.node_count();
    // Print degree distribution with percentages
    println!("Degree distribution:");
    for(degree, count) in &degree_distribution {
        let percentage = (*count as f64 / total_nodes as f64) * 100.0;
        println!("Degree {}: % of total nodes = {:.2}%", degree, percentage);
    }
    let components = graph.connected_components();

    for (i, component) in components.iter().enumerate() {
        let total_value: u64 = component.iter().map(|player| player.value_eur as u64).sum();
        let avg_value = total_value as f64 / component.len() as f64;
        let ovr = 94 - i;
        println!("Overall {} (size: {}): average market value (euros) = {:.2}", ovr, component.len(), avg_value);
    }
    plot_degree_distribution(&graph, "dd_plot.svg")?;

    Ok(())
}
