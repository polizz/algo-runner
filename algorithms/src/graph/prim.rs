use petgraph::{
  adj::NodeIndex, algo::min_spanning_tree, data::FromElements, dot::Dot, graph::UnGraph,
};
use std::collections::HashMap;
use std::fs;

pub struct MyPetGraph(UnGraph<String, f64, usize>);

impl From<&str> for MyPetGraph {
  fn from(graph_def: &str) -> Self {
    let mut graph = UnGraph::<_, f64, usize>::default();
    let f = fs::read_to_string(graph_def).unwrap();
    // let node_count: usize = f.lines().next().unwrap().parse().unwrap();
    let lines = f.lines().skip(2);
    // graph.reserve_nodes(node_count);
    let mut unique_nodes = HashMap::<String, _>::new();

    lines.for_each(|edge_def| {
      let mut vals = edge_def.split_whitespace();
      let from: usize = vals.next().unwrap().parse().unwrap();
      let to: usize = vals.next().unwrap().parse().unwrap();
      let weight: f64 = vals.next().unwrap().parse().unwrap();

      let from = format!("{from}");
      if !unique_nodes.contains_key(&from) {
        // println!("caching from: {}", &from);
        let node_idx = graph.add_node(from.clone());
        unique_nodes.insert(from.clone(), node_idx);
      };
      let from = unique_nodes.get(&from).unwrap().clone();

      let to = format!("{to}");
      if !unique_nodes.contains_key(&to) {
        // println!("caching to: {}", &to);
        let node_idx = graph.add_node(to.clone());
        unique_nodes.insert(to.clone(), node_idx);
      };
      let to = unique_nodes.get(&to).unwrap();

      // dbg!(&from);
      // dbg!(&to);

      graph.add_edge(from, to.clone(), weight);
    });

    // dbg!(&graph.node_count());
    MyPetGraph(graph)
  }
}

#[derive(Clone, Debug)]
struct Edge {
  weight: f64,
  to: usize,
}

#[derive(Debug)]
pub struct MyGraph {
  v_count: usize,
  edges: Vec<Vec<Edge>>,
}

impl MyGraph {
  pub fn new(graph_def: &str) -> Self {
    let f = fs::read_to_string(graph_def).unwrap();
    let mut lines = f.lines();
    let v_count: usize = lines.next().unwrap().parse().unwrap();
    let _edge_count = lines.next().unwrap();

    let mut g = MyGraph {
      v_count,
      edges: vec![vec![]; v_count],
    };

    lines.for_each(|edge_def| {
      let mut vals = edge_def.split_whitespace();
      let from: usize = vals.next().unwrap().parse().unwrap();
      let to: usize = vals.next().unwrap().parse().unwrap();
      let weight: f64 = vals.next().unwrap().parse().unwrap();

      g.edges[from].push(Edge { to, weight });
    });

    g
  }
}

pub struct Prim;

impl Prim {
  pub fn get_min_span_tree(g: MyGraph, start_vertex: usize) -> f64 {
    let mut in_tree = vec![false; g.v_count];
    let mut parent = vec![usize::MAX; g.v_count];
    let mut distance_to_start = vec![f64::MAX; g.v_count];
    let mut total_weight = 0.0;
    let mut v = start_vertex;
    let mut dist = 0.0;

    distance_to_start[start_vertex] = 0.0;

    while !in_tree[v] {
      in_tree[v] = true;

      if v != start_vertex {
        total_weight = total_weight + dist;
      }

      for edge in &g.edges[v] {
        if distance_to_start[edge.to] > edge.weight && !in_tree[edge.to] {
          // set distance to each vertext for each edge for this vertex
          distance_to_start[edge.to] = edge.weight;
          // set parent for each destination vertex for each edge for this vertex
          parent[edge.to] = v;
        }
      }
      // dbg!(&in_tree, &parent, &distance_to_start);

      dist = f64::MAX;
      for vertex in 0..g.v_count {
        // println!("{vertex}");
        // for all vertices not in tree, pick one with smallest
        if !in_tree[vertex] && dist > distance_to_start[vertex] {
          dist = distance_to_start[vertex];
          v = vertex;
        }
      }
    }

    // dbg!(parent);

    total_weight
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn petgraph_test() {
    let medium_wg = "src/graph/fixtures/tinyEWG.txt";
    // let medium_wg = "src/graph/fixtures/mediumEWG.txt";

    let mg = MyPetGraph::from(medium_wg);
    let mst = UnGraph::<_, _>::from_elements(min_spanning_tree(&mg.0));
    // dbg!(Dot::new(&mst));

    let total_weight: f64 = mst.edge_weights().sum();
    // dbg!(total_weight);
    println!("kruskal_weight:{}", &total_weight);
  }

  #[test]
  fn prim_test() {
    let medium_wg = "src/graph/fixtures/tinyEWG.txt";
    // let medium_wg = "src/graph/fixtures/mediumEWG.txt";

    let g = MyGraph::new(medium_wg);
    let weight = Prim::get_min_span_tree(g, 0);
    println!("prim_weight:{}", &weight);
  }
}
