use crate::chapter_9_graph::{Distance, Graph, Path, VertexId, Weight};
use priority_queue::PriorityQueue;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy)]
struct Waypoint {
    distance_from_departure: Distance,
    parent_vertex_id: VertexId,
}

pub(crate) struct DijkstraAlgorithm {
    priority_queue: PriorityQueue<VertexId, Weight>,
    visited_vertices: HashSet<VertexId>,
    shorter_waypoints: HashMap<VertexId, Waypoint>,
    departure: VertexId,
    destination: VertexId,
}

impl DijkstraAlgorithm {
    pub(crate) fn new(departure: VertexId, destination: VertexId) -> Self {
        let mut dt = DijkstraAlgorithm {
            priority_queue: PriorityQueue::new(),
            visited_vertices: HashSet::new(),
            shorter_waypoints: HashMap::new(),
            departure,
            destination,
        };

        dt.save_waypoint(
            departure,
            Waypoint {
                parent_vertex_id: departure,
                distance_from_departure: 0,
            },
        );

        dt
    }

    fn get_next(&mut self) -> Option<(VertexId, Weight)> {
        let (next_id, minus_weight) = self.priority_queue.pop()?;
        self.visited_vertices.insert(next_id);
        Some((next_id, -minus_weight))
    }

    fn save_waypoint(&mut self, waypoint_vertex: VertexId, waypoint: Waypoint) {
        let discovered_shorter_path_to_waypoint = if let Some(known_distance) = self
            .shorter_waypoints
            .get(&waypoint_vertex)
            .map(|v| v.distance_from_departure)
        {
            waypoint.distance_from_departure < known_distance
        } else {
            true
        };

        if discovered_shorter_path_to_waypoint && !self.visited_vertices.contains(&waypoint_vertex)
        {
            self.shorter_waypoints.insert(waypoint_vertex, waypoint);
            self.priority_queue
                .push(waypoint_vertex, -waypoint.distance_from_departure);
        };
    }

    fn populate_shorter_legs<T>(&mut self, graph: &Graph<T>) -> Option<()> {
        loop {
            let (parent_id, parent_distance_from_departure) = self.get_next()?;

            for get_neighbor in graph.get_vertex(&parent_id)?.get_neighbors() {
                self.save_waypoint(
                    get_neighbor.get_id(),
                    Waypoint {
                        parent_vertex_id: parent_id,
                        distance_from_departure: parent_distance_from_departure
                            + get_neighbor.weight,
                    },
                );
            }
        }
    }

    fn get_path<'a, T>(&self, graph: &'a Graph<T>) -> Option<Path<'a, T>> {
        let mut waypoints = vec![graph.get_vertex(&self.destination)?.vertex];
        let mut waypoint = self.destination;
        let distance = self
            .shorter_waypoints
            .get(&self.destination)?
            .distance_from_departure;

        loop {
            let travel = self.shorter_waypoints.get(&waypoint)?;
            waypoint = travel.parent_vertex_id;
            waypoints.push(graph.get_vertex(&waypoint)?.vertex);
            if waypoint == self.departure {
                break;
            }
        }
        waypoints.reverse();

        Some(Path {
            distance,
            waypoints,
        })
    }

    pub(crate) fn find_shortest_path<T>(mut self, graph: &Graph<T>) -> Option<Path<T>> {
        self.populate_shorter_legs(graph);
        self.get_path(graph)
    }
}
