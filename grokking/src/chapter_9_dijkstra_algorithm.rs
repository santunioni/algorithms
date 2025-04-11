use crate::chapter_9_graph::{Graph, Vertex, VertexId, Weight};
use priority_queue::PriorityQueue;
use std::collections::{HashMap, HashSet};

type Distance = Weight;

#[derive(Clone, Copy)]
struct Travel {
    weight: Distance,
    parent: VertexId,
}

struct DijkstraTable {
    priority_queue: PriorityQueue<VertexId, Weight>,
    visited_vertices: HashSet<VertexId>,
    travels: HashMap<VertexId, Travel>,
    departure: VertexId,
    destination: VertexId,
}

impl DijkstraTable {
    fn new(departure: VertexId, destination: VertexId) -> Self {
        let mut dt = DijkstraTable {
            priority_queue: PriorityQueue::new(),
            visited_vertices: HashSet::new(),
            travels: HashMap::new(),
            departure,
            destination,
        };

        dt.save_neighbor(
            departure,
            Travel {
                parent: departure,
                weight: 0,
            },
        );

        dt
    }

    fn get_next(&mut self) -> Option<(VertexId, Weight)> {
        let (next_id, minus_weight) = self.priority_queue.pop()?;
        self.visited_vertices.insert(next_id);
        Some((next_id, -minus_weight))
    }

    fn save_neighbor(&mut self, vertex: VertexId, new_travel: Travel) {
        let discovered_shorter_distance =
            if let Some(known_distance) = self.travels.get(&vertex).map(|v| v.weight) {
                new_travel.weight < known_distance
            } else {
                true
            };

        if discovered_shorter_distance && !self.visited_vertices.contains(&vertex) {
            self.travels.insert(vertex, new_travel);
            self.priority_queue.push(vertex, -new_travel.weight);
        };
    }

    fn get_path<'a, T>(&self, graph: &'a Graph<T>) -> Option<(Distance, Vec<&'a Vertex<T>>)> {
        let mut path = vec![graph.get_vertex(&self.destination)?.vertex];
        let mut stop_point = self.destination;
        let distance = self.travels.get(&self.destination)?.weight;

        loop {
            let travel = self.travels.get(&stop_point)?;
            stop_point = travel.parent;
            path.push(graph.get_vertex(&stop_point)?.vertex);
            if stop_point == self.departure {
                break;
            }
        }
        path.reverse();

        Some((distance, path))
    }
}

trait DijkstraAlgorithm<T> {
    fn find_shortest_route(
        &self,
        departure: VertexId,
        destination: VertexId,
    ) -> Option<(Distance, Vec<&Vertex<T>>)>;
}

impl<T> DijkstraAlgorithm<T> for Graph<T> {
    fn find_shortest_route(
        &self,
        departure: VertexId,
        destination: VertexId,
    ) -> Option<(Distance, Vec<&Vertex<T>>)> {
        let mut dijkstra_table = DijkstraTable::new(departure, destination);

        loop {
            let Some((parent_id, parent_weight)) = dijkstra_table.get_next() else {
                break;
            };

            for get_neighbor in self.get_vertex(&parent_id)?.get_neighbors() {
                dijkstra_table.save_neighbor(
                    get_neighbor.get_id(),
                    Travel {
                        parent: parent_id,
                        weight: parent_weight + get_neighbor.weight,
                    },
                );
            }
        }

        dijkstra_table.get_path(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::chapter_9_dijkstra_algorithm::DijkstraAlgorithm;
    use crate::chapter_9_graph::Graph;

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

        let (distance, path) = graph
            .find_shortest_route(start, finish)
            .expect("Should find a path");

        assert_eq!(distance, 6);
        assert_eq!(
            path,
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

        let (distance, path) = graph
            .find_shortest_route(book, piano)
            .expect("Should find a path");

        assert_eq!(distance, 35);
        assert_eq!(
            path,
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

        let (distance, _) = graph
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

        let (distance, _) = graph
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

        let (distance, _) = graph
            .find_shortest_route(start, finish)
            .expect("Should find a path");

        assert_eq!(distance, 4);
    }
}
