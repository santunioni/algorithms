use crate::chapter_9_graph::{Graph, Vertex, VertexId};
use std::collections::HashMap;

type Distance = f32;

#[derive(Clone, Copy)]
struct Travel {
    distance: Distance,
    from_node: VertexId,
}

struct DijkstraAlgorithm<'a, T> {
    graph: &'a Graph<T>,
}

impl<'a, T> DijkstraAlgorithm<'a, T> {
    fn new(graph: &'a Graph<T>) -> Self {
        DijkstraAlgorithm { graph }
    }

    fn find(
        &self,
        departure: VertexId,
        mut destination: VertexId,
    ) -> Option<(Distance, Vec<&'a Vertex<T>>)> {
        let default_travel = || Travel {
            from_node: departure,
            distance: Distance::MAX,
        };
        let mut travels = HashMap::new();
        travels.insert(
            departure,
            Travel {
                from_node: departure,
                distance: 0 as Distance,
            },
        );

        for get_vertex in self.graph.breath_search_iterator(&departure) {
            let vertex_travel = *travels
                .entry(*get_vertex.get_id())
                .or_insert_with(default_travel);

            for get_neighbor in get_vertex.get_neighbors() {
                let neighbor_travel = *travels
                    .entry(*get_neighbor.get_id())
                    .or_insert_with(default_travel);

                let new_travel = Travel {
                    from_node: *get_vertex.get_id(),
                    distance: vertex_travel.distance + get_neighbor.weight,
                };

                if new_travel.distance < neighbor_travel.distance {
                    travels.insert(*get_neighbor.get_id(), new_travel);
                }
            }
        }

        let mut path = vec![self.graph.get_vertex(&destination)?.vertex];
        let distance = travels.get(&destination)?.distance;

        loop {
            let travel = travels.get(&destination)?;
            destination = travel.from_node;
            path.push(self.graph.get_vertex(&destination)?.vertex);
            if destination == departure {
                break;
            }
        }
        path.reverse();

        Some((distance, path))
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

        graph.attach_weighted(&start, &a, 6.0);
        graph.attach_weighted(&start, &b, 2.0);
        graph.attach_weighted(&a, &finish, 1.0);
        graph.attach_weighted(&b, &a, 3.0);
        graph.attach_weighted(&b, &finish, 5.0);

        let dijkstra = DijkstraAlgorithm::new(&graph);

        let (distance, path) = dijkstra.find(start, finish).expect("Should find a path");

        assert_eq!(distance, 6.0);
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
}
