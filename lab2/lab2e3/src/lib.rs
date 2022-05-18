pub mod graph {
    use std::collections::HashMap;
    use std::fmt::Display;
    use crate::graph::graph_items::edge::Edge;
    use crate::graph::graph_items::node::Node;

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;
            use std::fmt::Display;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Edge {
                pub from: String,
                pub to: String,
                pub attrs: HashMap<String, String>,
            }

            impl Display for Edge {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{} -> {}", self.from, self.to)
                }
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Edge {
                    Edge {
                        from: String::from(from),
                        to: String::from(to),
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    attrs.iter().for_each(|(k, v)| {
                        self.attrs.insert(String::from(*k), String::from(*v));
                    });
                    self
                }
            }
        }
        pub mod node {
            use std::collections::HashMap;
            use std::fmt::Display;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Node{
                pub name: String,
                pub attrs: HashMap<String, String>
            }

            impl Display for Node {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}[{}]", self.name, self.attrs.iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<String>>().join(", "))
                }
            }

            impl Node{
                pub fn new(name: &str) -> Node {
                    Node{
                        name: String::from(name),
                        attrs: HashMap::new()
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    attrs.iter().for_each(|(k, v)| {
                        self.attrs.insert(String::from(*k), String::from(*v));
                    });
                    self
                }
                pub fn get_attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|s| s.as_str())
                }
            }
        }
    }

    pub struct Graph{
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: HashMap<String, String>
    }

    impl Display for Graph{
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "digraph {{\n")?;
            self.nodes.iter().for_each(|n| {
                write!(f, "{}\n", n).unwrap();
            });
            self.edges.iter().for_each(|e| {
                write!(f, "{}\n", e).unwrap();
            });
            write!(f, "}}")
        }
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new()
            }
        }
        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.to_vec();
            self
        }
        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.to_vec();
            self
        }
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            attrs.iter().for_each(|(k, v)| {
                self.attrs.insert(String::from(*k), String::from(*v));
            });
            self
        }
        pub fn get_node(&self, name: &str) -> Option<&graph_items::node::Node> {
            self.nodes.iter().find(|n| n.name == name)
        }
    }
}
