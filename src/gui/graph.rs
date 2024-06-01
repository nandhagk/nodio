use eframe::{self, egui};
use egui_nodes::{Context, LinkArgs, NodeArgs, NodeConstructor, PinArgs, PinShape};

pub enum NodeAttrType{
    STATIC, INPUT, OUTPUT
}

pub struct NodeAttr{
    attr_idx: usize,
    attr_type: NodeAttrType,
    text: String
}

impl NodeAttr{
    pub fn new(attr_type: NodeAttrType, text: &str) -> Self{
        NodeAttr{ attr_idx: 0, attr_type, text: String::from(text)}
    }
}

pub struct Node{
    title: String,
    node_idx: usize,
    attributes: Vec<NodeAttr>
}

impl Node{
    pub fn new(title: &str, attributes: Vec<NodeAttr>) -> Self{
        Node {title: String::from(title), node_idx: 0, attributes}
    }
}

pub struct Graph{
    links: Vec<(usize, usize)>,
    nodes: std::collections::HashMap<usize, Node>,
    node_idx_ctr: usize,
    attr_idx_ctr: usize
}

impl Graph{
    pub fn delete_selection(self: &mut Self, ctx: &mut Context){
        if ctx.get_selected_links().len() > 0{
            let links_to_remove: Vec<(usize, usize)> = ctx.get_selected_links().iter().map(|i| self.links[*i]).collect();
            self.links = self.links.iter().filter(|val| !links_to_remove.contains(val)).map(|val| (val.0, val.1)).collect();
        }
    
        for node_idx in ctx.get_selected_nodes().iter(){   
            for attr_idx in self.nodes[node_idx].attributes.iter().map(|attr| attr.attr_idx){
                self.links = self.links.iter().filter(|val| (val.0 != attr_idx) && (val.1 != attr_idx)).map(|val| (val.0, val.1)).collect();
            }
            self.nodes.remove(node_idx);
        }
        
    }
    pub fn add_node(self: &mut Self, mut node: Node){
        for attr in node.attributes.iter_mut(){
            if attr.attr_idx == 0
            {
                self.attr_idx_ctr += 1;
                attr.attr_idx = self.attr_idx_ctr;
            }
        }
        if node.node_idx == 0
        {
            self.node_idx_ctr += 1;
            node.node_idx = self.node_idx_ctr;
        }

        self.nodes.insert(node.node_idx, node);
    }

}

impl Default for Graph {
    fn default() -> Self {
        Self {
            links: Vec::default(),
            nodes: std::collections::HashMap::default(),
            node_idx_ctr: 0,
            attr_idx_ctr: 0
        }
    }
}

impl Graph{
    pub fn update(self: &mut Self, ctx: &mut Context, ui: &mut egui::Ui){
        let mut gnodes = Vec::<NodeConstructor>::with_capacity(self.nodes.len());
        for (_id, node) in self.nodes.iter(){
            let mut gnode = NodeConstructor::new(
                node.node_idx,
                NodeArgs::default()
            );
            gnode = gnode.with_title(|ui| ui.label(node.title.as_str()));
            
            for atr in node.attributes.iter(){
                match atr.attr_type {
                    NodeAttrType::STATIC => {
                        gnode = gnode.with_static_attribute(
                            atr.attr_idx, 
                            |ui| ui.label(atr.text.as_str())
                        );
                    }
                    NodeAttrType::INPUT => {
                        gnode = gnode.with_input_attribute(
                            atr.attr_idx, 
                            PinArgs{
                                shape: PinShape::Triangle,
                                ..Default::default()  
                            },
                            |ui| ui.label(atr.text.as_str())
                        );
                    }
                    NodeAttrType::OUTPUT => {
                        gnode = gnode.with_output_attribute(
                            atr.attr_idx,
                            PinArgs{
                                shape: PinShape::CircleFilled,
                                ..Default::default()
                            },
                            |ui| ui.label(atr.text.as_str())
                        );
                    }
                }
            }

            gnodes.push(gnode);
        }
        
        ctx.show(
            gnodes,
            self.links
                .iter()
                .enumerate()
                .map(|(i, (start, end))| (i, *start, *end, LinkArgs::default())),
            ui,
        );
        
        if let Some(idx) = ctx.link_destroyed() {
            self.links.swap_remove(idx);
        }
    
        // add created links
        if let Some((start, end, _)) = ctx.link_created() {
            self.links.push((start, end))
        }
    }
}
