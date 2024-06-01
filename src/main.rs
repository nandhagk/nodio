mod gui;
use gui::graph::{Graph, Node, NodeAttr, NodeAttrType};
fn main() {
    println!("Starting nodio :)");
    let graph = Graph::default();
    gui::show(
        "Hello World",
        graph,
        vec![
            (
                String::from("Amplifier"),
                Box::new(|| {
                    Node::new(
                        "Amplifier",
                        vec![
                            NodeAttr::new(NodeAttrType::INPUT, "Input Signal"),
                            NodeAttr::new(NodeAttrType::STATIC, ""),
                            NodeAttr::new(NodeAttrType::OUTPUT, "Output Signal"),
                        ],
                    )
                }),
            ),
            (
                String::from("Audio Input"),
                Box::new(|| {
                    Node::new(
                        "Audio Input",
                        vec![NodeAttr::new(NodeAttrType::OUTPUT, "Microphone")],
                    )
                }),
            ),
        ],
    );
}
