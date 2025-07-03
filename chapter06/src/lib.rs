use petgraph::{visit::{Bfs, Dfs}, Graph};


pub fn graph_sample() {

    println!("Graph Sample");
    let mut graph = Graph::new_undirected();

    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c =  graph.add_node("C");
    let d =  graph.add_node("D");
    let e =  graph.add_node("E");

    graph.add_edge(a, b, "");
    graph.add_edge(a, c, "");
    graph.add_edge(b, d, "");
    graph.add_edge(c,e,  "");

    println!("DFS:");
    let mut dfs = Dfs::new(&graph, a);
    while let Some(nx) = dfs.next(&graph) {
        println!("Visited: {}", graph[nx]);
    }

    println!("\nBFS:");
    let mut bfs = Bfs::new(&graph, a);
    while let Some(nx) = bfs.next(&graph) {
        println!("Visited: {}", graph[nx]);
    }
}
