use crate::chapter_5_hashset::HashSet;
use crate::chapter_9_dijkstra_algorithm::DijkstraAlgorithm;
use std::collections::{HashMap, VecDeque};

pub type VertexId = usize;
pub type Weight = isize;
pub type Distance = Weight;

pub struct Path<'a, T> {
    pub distance: Distance,
    pub waypoints: Vec<&'a Vertex<T>>,
}

pub struct Leg {
    to_vertex_id: VertexId,
    weight: Weight,
}

#[derive(Debug, PartialEq)]
pub struct Vertex<T> {
    id: VertexId,
    item: T,
}

impl<T> Vertex<T> {
    pub fn get_id(&self) -> VertexId {
        self.id
    }

    pub fn get_item(&self) -> &T {
        &self.item
    }
}

pub struct Graph<T> {
    edges: HashMap<VertexId, Vec<Leg>>,
    vertices: HashMap<VertexId, Vertex<T>>,
    counter: VertexId,
}

impl<T> Graph<T> {
    pub fn new() -> Self {
        Graph {
            edges: HashMap::new(),
            vertices: HashMap::new(),
            counter: 0,
        }
    }

    pub fn add_vertex(&mut self, item: T) -> VertexId {
        let id = self.counter;
        self.counter += 1;
        let vertex = Vertex { item, id };
        self.vertices.insert(id, vertex);
        id
    }

    pub fn get_vertex(&self, vertex_id: &VertexId) -> Option<GetVertex<T>> {
        let vertex = self.vertices.get(vertex_id);
        vertex.map(|vertex| GetVertex {
            vertex,
            graph: self,
        })
    }

    pub fn attach_weighted(&mut self, from: &VertexId, to: &VertexId, weight: Weight) {
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

    pub fn attach(&mut self, from: &VertexId, to: &VertexId) {
        self.attach_weighted(from, to, 1 as Weight);
    }

    pub fn depth_search_iterator(
        &self,
        start: &VertexId,
    ) -> impl Iterator<Item = GetVertex<'_, T>> {
        GraphIterator::new(start, Mode::Depth, self)
    }

    pub fn breath_search_iterator(
        &self,
        start: &VertexId,
    ) -> impl Iterator<Item = GetVertex<'_, T>> {
        GraphIterator::new(start, Mode::Breath, self)
    }

    fn find_shortest_route(&self, departure: VertexId, destination: VertexId) -> Option<Path<T>> {
        DijkstraAlgorithm::new(departure, destination).find_shortest_path(self)
    }
}

pub struct GetVertex<'a, T> {
    pub vertex: &'a Vertex<T>,
    graph: &'a Graph<T>,
}

pub struct GetNeighbor<'a, T> {
    get_vertex: GetVertex<'a, T>,
    pub weight: Weight,
}

impl<'a, T> GetVertex<'a, T> {
    pub fn get_neighbors(&self) -> Vec<GetNeighbor<'a, T>> {
        let mut neighbors = Vec::new();
        let Some(legs) = self.graph.edges.get(&self.vertex.id) else {
            return neighbors;
        };
        for leg in legs {
            let Some(vertex) = self.graph.get_vertex(&leg.to_vertex_id) else {
                continue;
            };
            neighbors.push(GetNeighbor {
                weight: leg.weight,
                get_vertex: vertex,
            })
        }
        neighbors.sort_by(|a, b| a.weight.cmp(&b.weight));
        neighbors
    }

    pub fn get_item(&self) -> &T {
        &self.vertex.item
    }

    pub fn get_id(&self) -> &VertexId {
        &self.vertex.id
    }
}

impl<T> GetNeighbor<'_, T> {
    pub fn get_item(&self) -> &T {
        self.get_vertex.get_item()
    }
    pub fn get_id(&self) -> VertexId {
        self.get_vertex.vertex.get_id()
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
    type Item = GetVertex<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_vertex_id = self.queue.pop_front()?;
        let next_vertex = self.graph.get_vertex(&next_vertex_id)?;

        for get_neighbor in next_vertex.get_neighbors() {
            self.put_to_queue(&get_neighbor.get_vertex.vertex.id);
        }

        Some(GetVertex {
            vertex: next_vertex.vertex,
            graph: self.graph,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::chapter_9_graph::{Graph, Path};

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
                .map(|neighbor| neighbor.get_vertex.get_item())
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

        assert_eq!(iter.next().unwrap().get_item(), "Vinícius");
        assert_eq!(iter.next().unwrap().get_item(), "Bianca");
        assert!(iter.next().is_none());
    }

    #[test]
    fn should_find_shortest_path() {
        let mut graph = Graph::new();
        let start = graph.add_vertex("Start".to_string());
        let a = graph.add_vertex("A".to_string());
        let b = graph.add_vertex("B".to_string());
        let finish = graph.add_vertex("Finish".to_string());

        graph.attach_weighted(&start, &a, 6);
        graph.attach_weighted(&start, &b, 2);
        graph.attach_weighted(&a, &finish, 1);
        graph.attach_weighted(&b, &a, 3);
        graph.attach_weighted(&b, &finish, 5);

        let Path {
            distance,
            waypoints,
        } = graph
            .find_shortest_route(start, finish)
            .expect("Should find a path");

        assert_eq!(distance, 6);
        assert_eq!(
            waypoints,
            vec![
                graph.get_vertex(&start).unwrap().vertex,
                graph.get_vertex(&b).unwrap().vertex,
                graph.get_vertex(&a).unwrap().vertex,
                graph.get_vertex(&finish).unwrap().vertex,
            ]
        )
    }

    #[test]
    fn should_find_cheapest_trade_to_piano() {
        let mut graph = Graph::new();

        let book = graph.add_vertex("Book".to_string());
        let poster = graph.add_vertex("Poster".to_string());
        let drums = graph.add_vertex("Drum Set".to_string());
        let lp = graph.add_vertex("Rare LP".to_string());
        let bass = graph.add_vertex("Bass Guitar".to_string());
        let piano = graph.add_vertex("Piano".to_string());

        graph.attach_weighted(&book, &poster, 0);
        graph.attach_weighted(&poster, &drums, 35);
        graph.attach_weighted(&drums, &piano, 10);

        graph.attach_weighted(&book, &lp, 5);
        graph.attach_weighted(&lp, &bass, 15);
        graph.attach_weighted(&bass, &piano, 20);

        graph.attach_weighted(&poster, &bass, 30);
        graph.attach_weighted(&lp, &drums, 20);

        let Path {
            distance,
            waypoints,
        } = graph
            .find_shortest_route(book, piano)
            .expect("Should find a path");

        assert_eq!(distance, 35);
        assert_eq!(
            waypoints,
            vec![
                graph.get_vertex(&book).unwrap().vertex,
                graph.get_vertex(&lp).unwrap().vertex,
                graph.get_vertex(&drums).unwrap().vertex,
                graph.get_vertex(&piano).unwrap().vertex,
            ]
        )
    }

    #[test]
    fn should_find_path_for_grokking_exercise_9_1_a() {
        let mut graph = Graph::new();

        let start = graph.add_vertex("Start".to_string());
        let a = graph.add_vertex("A".to_string());
        let b = graph.add_vertex("B".to_string());
        let c = graph.add_vertex("C".to_string());
        let d = graph.add_vertex("D".to_string());
        let finish = graph.add_vertex("Finish".to_string());

        graph.attach_weighted(&start, &a, 5);
        graph.attach_weighted(&start, &b, 2);

        graph.attach_weighted(&a, &c, 4);
        graph.attach_weighted(&a, &d, 2);

        graph.attach_weighted(&b, &a, 8);
        graph.attach_weighted(&b, &d, 7);

        graph.attach_weighted(&c, &d, 6);
        graph.attach_weighted(&c, &finish, 3);

        graph.attach_weighted(&d, &finish, 1);

        let Path { distance, .. } = graph
            .find_shortest_route(start, finish)
            .expect("Should find a path");

        assert_eq!(distance, 8);
    }

    #[test]
    fn should_find_path_for_grokking_exercise_9_1_b() {
        let mut graph = Graph::new();

        let start = graph.add_vertex("Start".to_string());
        let a = graph.add_vertex("A".to_string());
        let b = graph.add_vertex("B".to_string());
        let c = graph.add_vertex("C".to_string());
        let finish = graph.add_vertex("Finish".to_string());

        graph.attach_weighted(&start, &a, 10);
        graph.attach_weighted(&a, &c, 20);
        graph.attach_weighted(&c, &b, 1);
        graph.attach_weighted(&b, &a, 1);
        graph.attach_weighted(&c, &finish, 30);

        let Path { distance, .. } = graph
            .find_shortest_route(start, finish)
            .expect("Should find a path");

        assert_eq!(distance, 60);
    }

    #[test]
    fn should_find_path_for_grokking_exercise_9_1_c() {
        let mut graph = Graph::new();

        let start = graph.add_vertex("Start".to_string());
        let a = graph.add_vertex("A".to_string());
        let b = graph.add_vertex("B".to_string());
        let c = graph.add_vertex("C".to_string());
        let finish = graph.add_vertex("Finish".to_string());

        graph.attach_weighted(&start, &a, 2);
        graph.attach_weighted(&start, &b, 2);
        graph.attach_weighted(&b, &a, 2);
        graph.attach_weighted(&a, &c, 2);
        graph.attach_weighted(&a, &finish, 2);
        graph.attach_weighted(&c, &finish, 2);
        graph.attach_weighted(&c, &b, -1);

        let Path { distance, .. } = graph
            .find_shortest_route(start, finish)
            .expect("Should find a path");

        assert_eq!(distance, 4);
    }
}
