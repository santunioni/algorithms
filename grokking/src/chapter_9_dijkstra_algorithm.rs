use crate::chapter_9_graph::{Distance, Graph, Path, VertexId, Weight};
use priority_queue::PriorityQueue;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy)]
struct Waypoint {
    distance_from_departure: Distance,
    parent_vertex_id: VertexId,
}

pub(crate) struct DijkstraAlgorithm<'a, T> {
    priority_queue: PriorityQueue<VertexId, Weight>,
    visited_vertices: HashSet<VertexId>,
    shorter_waypoints: HashMap<VertexId, Waypoint>,
    departure: VertexId,
    destination: VertexId,
    graph: &'a Graph<T>,
}

impl<'a, T> DijkstraAlgorithm<'a, T> {
    pub(crate) fn new(graph: &'a Graph<T>, departure: VertexId, destination: VertexId) -> Self {
        let mut dt = DijkstraAlgorithm {
            priority_queue: PriorityQueue::new(),
            visited_vertices: HashSet::new(),
            shorter_waypoints: HashMap::new(),
            departure,
            destination,
            graph,
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

    pub(crate) fn into_shortest_path(mut self) -> Option<Path<'a, T>> {
        self.populate_shorter_legs();
        self.get_path()
    }

    fn populate_shorter_legs(&mut self) -> Option<()> {
        loop {
            let (parent_id, parent_distance_from_departure) = self.get_next()?;

            if parent_id == self.destination {
                return None;
            }

            for get_neighbor in self.graph.get_vertex(&parent_id)?.get_neighbors() {
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

    fn get_path(&self) -> Option<Path<'a, T>> {
        let mut waypoints = vec![self.graph.get_vertex(&self.destination)?.vertex];
        let mut waypoint = self.destination;
        let distance = self
            .shorter_waypoints
            .get(&self.destination)?
            .distance_from_departure;

        loop {
            let travel = self.shorter_waypoints.get(&waypoint)?;
            waypoint = travel.parent_vertex_id;
            waypoints.push(self.graph.get_vertex(&waypoint)?.vertex);
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
}
