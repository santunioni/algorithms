use crate::chapter_6_graph::{Distance, Graph, Path, VertexId, Weight};
use priority_queue::PriorityQueue;
use std::collections::{HashMap, HashSet};

struct Waypoint {
    distance: Distance,
    parent: VertexId,
    vertex: VertexId,
}

pub(crate) struct DijkstraAlgorithm<'a, T> {
    /// Stores the vertex id with its priority, which is minus the distance from the departure vertex.
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

        dt.save_waypoint(Waypoint {
            vertex: departure,
            parent: departure,
            distance: 0,
        });

        dt
    }

    pub(crate) fn into_shortest_path(mut self) -> Option<Path<'a, T>> {
        self.look_for_waypoints();
        self.get_path()
    }

    #[inline]
    fn look_for_waypoints(&mut self) -> Option<()> {
        loop {
            let (parent, parent_distance_from_departure) =
                self.get_vertex_closer_to_departure_not_visited_yet()?;

            if parent == self.destination {
                return None;
            }

            for get_neighbor in self.graph.get_vertex(&parent)?.get_neighbors() {
                let waypoint = Waypoint {
                    vertex: get_neighbor.get_id(),
                    parent,
                    distance: parent_distance_from_departure + get_neighbor.weight,
                };

                if self.discovered_shorter_path(&waypoint)
                    && !self.visited_vertices.contains(&waypoint.vertex)
                {
                    self.save_waypoint(waypoint);
                };
            }
        }
    }

    #[inline]
    fn get_vertex_closer_to_departure_not_visited_yet(&mut self) -> Option<(VertexId, Weight)> {
        let (next_id, minus_distance) = self.priority_queue.pop()?;
        self.visited_vertices.insert(next_id);
        Some((next_id, -minus_distance))
    }

    #[inline]
    fn discovered_shorter_path(&self, waypoint: &Waypoint) -> bool {
        if let Some(known_distance) = self
            .shorter_waypoints
            .get(&waypoint.vertex)
            .map(|v| v.distance)
        {
            waypoint.distance < known_distance
        } else {
            true
        }
    }

    #[inline]
    fn save_waypoint(&mut self, waypoint: Waypoint) {
        self.priority_queue
            .push(waypoint.vertex, -waypoint.distance);
        self.shorter_waypoints.insert(waypoint.vertex, waypoint);
    }

    #[inline]
    fn get_path(&self) -> Option<Path<'a, T>> {
        let mut waypoints = vec![self.graph.get_vertex(&self.destination)?.vertex];
        let mut waypoint = self.destination;
        let distance = self.shorter_waypoints.get(&self.destination)?.distance;

        loop {
            let travel = self.shorter_waypoints.get(&waypoint)?;
            waypoint = travel.parent;
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
