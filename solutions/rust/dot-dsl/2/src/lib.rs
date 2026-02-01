pub mod graph {
    use graph_items::node::Node;
    use graph_items::edge::Edge;
    use std::collections::HashMap;
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
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
            self.attrs = attrs.iter().map(|&(k, v)| (k.to_string(), v.to_string())).collect();
            self
        }

        pub fn node(&self, key: &str) -> Option<&Node> {
            self.nodes.iter().find(|&x| x.name == key)
        }
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;
            #[derive(Debug, Clone, PartialEq)]
            pub struct Edge {
                node1: String,
                node2: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(node1: &str, node2: &str) -> Self {
                    Self {
                        node1: node1.to_string(), 
                        node2: node2.to_string(),
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs.iter().map(|&(k, v)| (k.to_string(), v.to_string())).collect();
                    self
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|x| x.as_str())
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;
            #[derive(Debug, Clone, PartialEq)]
            pub struct Node {
                pub name: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Self {
                        name: name.to_string(), 
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs.iter().map(|&(k, v)| (k.to_string(), v.to_string())).collect();
                    self
                }

                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|x| x.as_str())
                }
            }
        }

    }
}
