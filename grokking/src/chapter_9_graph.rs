use crate::chapter_5_hashset::HashSet;
use std::collections::{HashMap, VecDeque};

pub type VertexId = u64;
pub type Weight = f32;

pub struct Leg {
    to_vertex_id: VertexId,
    weight: Weight,
}

pub struct Vertex<T> {
    id: VertexId,
    item: T,
}

pub struct Neighbor<'a, T> {
    vertex: &'a Vertex<T>,
    weight: Weight,
}

pub struct GetVertex<'a, T> {
    vertex: &'a Vertex<T>,
    graph: &'a Graph<T>,
}

impl<'a, T> GetVertex<'a, T> {
    fn get_neighbors(&self) -> Vec<Neighbor<'a, T>> {
        let legs = &self.graph.edges[&self.vertex.id];
        let neighbors = legs.iter().map(|leg| Neighbor {
            weight: leg.weight,
            vertex: &self.graph.vertices[&leg.to_vertex_id],
        });
        neighbors.collect()
    }
}

pub struct Graph<T> {
    edges: HashMap<VertexId, Vec<Leg>>,
    vertices: HashMap<VertexId, Vertex<T>>,
    counter: VertexId,
}

impl<T> Graph<T> {
    fn new() -> Self {
        Graph {
            edges: HashMap::new(),
            vertices: HashMap::new(),
            counter: 0,
        }
    }

    fn add_vertex(&mut self, item: T) -> VertexId {
        let id = self.counter;
        self.counter += 1;
        let vertex = Vertex { item, id };
        self.vertices.insert(id, vertex);
        id
    }

    fn get_vertex(&self, vertex_id: &VertexId) -> Option<GetVertex<T>> {
        let vertex = self.vertices.get(vertex_id);
        vertex.map(|vertex| GetVertex {
            vertex,
            graph: self,
        })
    }

    fn attach_weighted(&mut self, from: &VertexId, to: &VertexId, weight: Weight) {
        let leg = Leg {
            weight,
            to_vertex_id: *to,
        };
        match self.edges.get_mut(from) {
            None => {
                self.edges.insert(*from, vec![leg]);
            }
            Some(vec) => {
                vec.push(leg);
            }
        }
    }

    fn attach(&mut self, from: &VertexId, to: &VertexId) {
        self.attach_weighted(from, to, 1 as Weight);
    }

    fn depth_search_iterator(&self, start: &VertexId) -> GraphIterator<T> {
        GraphIterator::new(start, Mode::Depth, self)
    }

    fn breath_search_iterator(&self, start: &VertexId) -> GraphIterator<T> {
        GraphIterator::new(start, Mode::Breath, self)
    }
}

enum Mode {
    Depth,
    Breath,
}

struct GraphIterator<'a, T> {
    queue: VecDeque<VertexId>,
    visited: HashSet<VertexId>,
    mode: Mode,
    graph: &'a Graph<T>,
}

impl<'a, T> GraphIterator<'a, T> {
    fn new(start: &VertexId, mode: Mode, graph: &'a Graph<T>) -> Self {
        let mut it = GraphIterator {
            queue: VecDeque::new(),
            visited: HashSet::new(),
            mode,
            graph,
        };
        it.put_to_queue(start);
        it
    }

    fn put_to_queue(&mut self, vertex_id: &VertexId) {
        if !self.visited.contains(vertex_id) {
            self.visited.insert(*vertex_id);
            match &self.mode {
                Mode::Breath => self.queue.push_back(*vertex_id),
                Mode::Depth => self.queue.push_front(*vertex_id),
            }
        }
    }
}

impl<'a, T> Iterator for GraphIterator<'a, T> {
    type Item = &'a Vertex<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_vertex_id = self.queue.pop_front()?;
        let next_vertex = self.graph.vertices.get(&next_vertex_id)?;

        if let Some(legs) = self.graph.edges.get(&next_vertex.id) {
            for leg in legs {
                self.put_to_queue(&leg.to_vertex_id);
            }
        };

        Some(next_vertex)
    }
}

#[cfg(test)]
mod tests {
    use crate::chapter_9_graph::Graph;

    #[test]
    fn should_build_node() {
        let mut graph = Graph::new();
        let vertex = graph.add_vertex(1);
        let get_vertex = graph.get_vertex(&vertex).unwrap();

        assert_eq!(get_vertex.vertex.item, 1);
    }

    #[test]
    fn should_build_graph() {
        let mut graph = Graph::new();
        let vini = graph.add_vertex("Vinícius".to_string());
        let bibi = graph.add_vertex("Bianca".to_string());

        graph.attach(&vini, &bibi);
        graph.attach(&bibi, &vini);

        assert_eq!(
            graph
                .get_vertex(&vini)
                .unwrap()
                .get_neighbors()
                .iter()
                .map(|neighbor| &neighbor.vertex.item)
                .collect::<Vec<_>>(),
            vec!["Bianca"]
        );
    }

    #[test]
    fn should_iter_on_nodes() {
        let mut graph = Graph::new();
        let vini = graph.add_vertex("Vinícius".to_string());
        let bibi = graph.add_vertex("Bianca".to_string());

        graph.attach(&vini, &bibi);
        graph.attach(&bibi, &vini);

        let mut iter = graph.breath_search_iterator(&vini);

        assert_eq!(iter.next().unwrap().item, "Vinícius");
        assert_eq!(iter.next().unwrap().item, "Bianca");
        assert!(iter.next().is_none());
    }
}
