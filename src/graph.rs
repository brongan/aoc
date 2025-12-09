pub trait Edge: Copy {
    fn first_vertex(&self) -> usize;
    fn second_vertex(&self) -> usize;
}

pub trait Graph {
    type E: Edge;
    fn edges_ordered_by_weight(&self) -> Vec<Self::E>;
    fn number_vertices(&self) -> usize;
}

use disjoint::DisjointSet;

pub fn kruskal<G: Graph>(graph: &G, unions: usize) -> DisjointSet {
    let mut result_edges = Vec::new();
    let mut vertices = DisjointSet::with_len(graph.number_vertices());
    let mut count = 0;

    for edge in graph.edges_ordered_by_weight() {
        if !vertices.is_joined(edge.first_vertex(), edge.second_vertex()) {
            vertices.join(edge.first_vertex(), edge.second_vertex());
            result_edges.push(edge);
        }
        count += 1;
        if count > unions {
            break;
        }
    }
    vertices
}
