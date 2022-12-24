use graph_items::edge::Edge;
use graph_items::node::Node;
use std::collections::HashMap;

pub mod graph_items {
    pub mod edge {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Edge {
            pub from: String,
            pub to: String,
            pub attrs: std::collections::HashMap<String, String>,
        }
    }

    pub mod node {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Node {
            pub attrs: std::collections::HashMap<String, String>,
            pub name: String,
        }
    }
}

#[derive(Default, Debug)]
pub struct Graph {
    pub attrs: HashMap<String, String>,
    pub edges: Vec<Edge>,
    pub nodes: Vec<Node>,
}

fn attrs_map(attrs: &[(&str, &str)]) -> HashMap<String, String> {
    attrs
        .iter()
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect()
}

impl Edge {
    pub fn new(a: &str, b: &str) -> Self {
        Edge {
            attrs: HashMap::new(),
            from: a.to_string(),
            to: b.to_string(),
        }
    }

    pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
        Self {
            attrs: attrs_map(attrs),
            ..self
        }
    }

    pub fn attr(&self, key: &str) -> Option<&str> {
        self.attrs.get(key).map(|x| x.as_str())
    }
}

impl Node {
    pub fn new(name: &str) -> Self {
        Node {
            name: name.to_string(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
        Node {
            attrs: attrs_map(attrs),
            ..self
        }
    }

    pub fn attr(&self, key: &str) -> Option<&str> {
        self.attrs.get(key).map(|x| x.as_str())
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

    pub fn node(&self, key: &str) -> Option<&Node> {
        self.nodes.iter().find(|n| n.name == key)
    }

    pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
        Self {
            attrs: attrs_map(attrs),
            ..self
        }
    }

    pub fn attr(&self, key: &str) -> Option<&str> {
        self.attrs.get(key).map(|x| x.as_str())
    }

    pub fn with_edges(self, edges: &[Edge]) -> Self {
        Self {
            edges: edges.to_vec(),
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

#[test]
fn test_graph_with_one_attribute() {
    let graph = Graph::new().with_attrs(&[("foo", "1")]);

    let expected_attrs: HashMap<String, String> =
        [("foo".to_string(), "1".to_string())].into_iter().collect();

    assert!(graph.nodes.is_empty());

    assert!(graph.edges.is_empty());

    assert_eq!(graph.attrs, expected_attrs);
}

#[test]
fn test_edges_store_attributes() {
    let nodes = vec![
        Node::new("a").with_attrs(&[("color", "green")]),
        Node::new("c"),
        Node::new("b").with_attrs(&[("label", "Beta!")]),
    ];

    let edges = vec![
        Edge::new("b", "c"),
        Edge::new("a", "b").with_attrs(&[("color", "blue"), ("fill", "darkblue")]),
    ];

    let attrs = vec![("foo", "1"), ("title", "Testing Attrs"), ("bar", "true")];

    let graph = Graph::new()
        .with_nodes(&nodes)
        .with_edges(&edges)
        .with_attrs(&attrs);

    assert_eq!(
        graph.edges,
        vec![
            Edge::new("b", "c"),
            Edge::new("a", "b").with_attrs(&[("color", "blue"), ("fill", "darkblue")]),
        ]
    );

    assert_eq!(graph.edges[1].attr("color"), Some("blue"));

    assert_eq!(graph.edges[1].attr("fill"), Some("darkblue"));

    assert_eq!(graph.edges[1].attr("foo"), None);

    assert_eq!(graph.edges[0].attr("color"), None);

    assert_eq!(graph.edges[0].attr("fill"), None);

    assert_eq!(graph.edges[0].attr("foo"), None);
}

#[test]
fn test_graph_nodes_store_attributes() {
    let attributes = [("foo", "bar"), ("bat", "baz"), ("bim", "bef")];

    let graph = Graph::new().with_nodes(
        &["a", "b", "c"]
            .iter()
            .zip(attributes.iter())
            .map(|(name, &attr)| Node::new(name).with_attrs(&[attr]))
            .collect::<Vec<_>>(),
    );

    let a = graph.node("a").expect("node a must be stored");

    assert_eq!(a.attr("foo"), Some("bar"));

    assert_eq!(a.attr("bat"), None);

    assert_eq!(a.attr("bim"), None);

    let b = graph.node("b").expect("node b must be stored");

    assert_eq!(b.attr("foo"), None);

    assert_eq!(b.attr("bat"), Some("baz"));

    assert_eq!(b.attr("bim"), None);

    let c = graph.node("c").expect("node c must be stored");

    assert_eq!(c.attr("foo"), None);

    assert_eq!(c.attr("bat"), None);

    assert_eq!(c.attr("bim"), Some("bef"));
}
