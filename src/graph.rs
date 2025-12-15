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

pub fn kruskal_with_limit<G: Graph>(graph: &G, unions: usize) -> DisjointSet {
    let mut vertices = DisjointSet::with_len(graph.number_vertices());
    let mut count = 0;

    for edge in graph.edges_ordered_by_weight() {
        if !vertices.is_joined(edge.first_vertex(), edge.second_vertex()) {
            vertices.join(edge.first_vertex(), edge.second_vertex());
        }
        count += 1;
        if count > unions {
            break;
        }
    }
    vertices
}

pub fn kruskal<G: Graph>(graph: &G) -> Vec<G::E> {
    let mut edges = Vec::new();
    let mut vertices = DisjointSet::with_len(graph.number_vertices());

    for edge in graph.edges_ordered_by_weight() {
        if !vertices.is_joined(edge.first_vertex(), edge.second_vertex()) {
            vertices.join(edge.first_vertex(), edge.second_vertex());
            edges.push(edge);
        }
    }
    edges
}
