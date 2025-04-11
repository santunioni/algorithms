use crate::chapter_9_graph::{Graph, Vertex, VertexId};
use std::collections::HashMap;

type Distance = f32;

#[derive(Clone, Copy)]
struct Travel {
    distance: Distance,
    from_node: VertexId,
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
        let destination = self.get_vertex(&destination)?.vertex;
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

        // Stop criteria for avoid searching very large graphs for alternative routes
        let mut how_many_times_reached_destination = 0;
        // Stop criteria for avoid searching very large graphs for alternative routes
        let mut how_many_iterations_after_reaching_destination = 0;

        for get_vertex in self.breath_search_iterator(&departure) {
            let vertex_travel = *travels
                .entry(*get_vertex.get_id())
                .or_insert_with(default_travel);

            for get_neighbor in get_vertex.get_neighbors() {
                let known_travel = *travels
                    .entry(get_neighbor.get_id())
                    .or_insert_with(default_travel);

                let discovered_travel = Travel {
                    from_node: *get_vertex.get_id(),
                    distance: vertex_travel.distance + get_neighbor.weight,
                };

                if discovered_travel.distance < known_travel.distance {
                    travels.insert(get_neighbor.get_id(), discovered_travel);
                }

                if get_neighbor.get_id() == destination.get_id() {
                    how_many_times_reached_destination += 1;
                }
            }

            if how_many_times_reached_destination > 0 {
                how_many_iterations_after_reaching_destination += 1;
            }

            // Stop criteria for avoid searching very large graphs for alternative routes
            if how_many_times_reached_destination > 10
                || how_many_iterations_after_reaching_destination > 100
            {
                break;
            }
        }

        let mut path = vec![destination];
        let mut stop_point = destination.get_id();
        let distance = travels.get(&destination.get_id())?.distance;

        loop {
            let travel = travels.get(&stop_point)?;
            stop_point = travel.from_node;
            path.push(self.get_vertex(&stop_point)?.vertex);
            if stop_point == departure {
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

        let (distance, path) = graph
            .find_shortest_route(start, finish)
            .expect("Should find a path");

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

    #[test]
    fn should_find_cheapest_trade_to_piano() {
        let mut graph = Graph::new();

        let book = graph.add_vertex("Book".to_string());
        let poster = graph.add_vertex("Poster".to_string());
        let drums = graph.add_vertex("Drum Set".to_string());
        let lp = graph.add_vertex("Rare LP".to_string());
        let bass = graph.add_vertex("Bass Guitar".to_string());
        let piano = graph.add_vertex("Piano".to_string());

        graph.attach_weighted(&book, &poster, 0.0);
        graph.attach_weighted(&poster, &drums, 35.0);
        graph.attach_weighted(&drums, &piano, 10.0);

        graph.attach_weighted(&book, &lp, 5.0);
        graph.attach_weighted(&lp, &bass, 15.0);
        graph.attach_weighted(&bass, &piano, 20.0);

        graph.attach_weighted(&poster, &bass, 30.0);
        graph.attach_weighted(&lp, &drums, 20.0);

        let (distance, path) = graph
            .find_shortest_route(book, piano)
            .expect("Should find a path");

        assert_eq!(distance, 35.0);
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
}
