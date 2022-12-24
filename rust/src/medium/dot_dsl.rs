#[derive(Debug)]
pub struct Edge {
    attrs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    attrs: Vec<String>,
    name: String,
}

#[derive(Default, Debug)]
pub struct Graph {
    attrs: Vec<String>,
    edges: Vec<Edge>,
    nodes: Vec<Node>,
}

impl Edge {
    pub fn new() -> Self {
        Edge { attrs: vec![] }
    }
}

impl Node {
    pub fn new(name: &str) -> Self {
        Node {
            name: name.to_string(),
            attrs: vec![],
        }
    }
}

impl Graph {
    pub fn new() -> Self {
        Graph::default()
    }

    pub fn with_nodes(self, nodes: &[Node]) -> Self {
        Graph {
            nodes: nodes.to_vec(),
            ..self
        }
    }
}

#[test]
fn test_empty_graph() {
    let graph = Graph::new();

    assert!(graph.nodes.is_empty());

    assert!(graph.edges.is_empty());

    assert!(graph.attrs.is_empty());
}

#[test]
fn test_graph_with_one_node() {
    let nodes = vec![Node::new("a")];

    let graph = Graph::new().with_nodes(&nodes);

    assert!(graph.edges.is_empty());

    assert!(graph.attrs.is_empty());

    assert_eq!(graph.nodes, vec![Node::new("a")]);
}
