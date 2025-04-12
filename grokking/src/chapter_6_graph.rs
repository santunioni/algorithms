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
}
