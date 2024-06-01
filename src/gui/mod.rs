use eframe;
use egui_nodes::Context;
use graph::Node;
pub mod graph;

pub struct App {
    ctx: Context,
    graph: graph::Graph,
    supported_new_nodes: Vec<(String, Box<fn() -> Node>)>,
    selected_new_node: usize
}

impl Default for App {
    fn default() -> Self {
        Self {
            ctx: Context::default(),
            graph: graph::Graph::default(),
            supported_new_nodes: Vec::default(),
            selected_new_node: 0
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            egui::ComboBox::from_label("Select transformer!").show_index(
                ui,
                &mut self.selected_new_node,
                self.supported_new_nodes.len(),
                |i| self.supported_new_nodes[i].0.as_str()
            );
            if ui.add(egui::Button::new("Add")).clicked() {
                self.graph.add_node(self.supported_new_nodes[self.selected_new_node].1());
                println!("Added {}", self.supported_new_nodes[self.selected_new_node].0);
            }
            if ui.add(egui::Button::new("Delete")).clicked() {
                self.graph.delete_selection(&mut self.ctx);
                println!("Removed selection");
            }
            self.graph.update(&mut self.ctx, ui);
        });
    }
}


pub fn show(app_name: &str, graph: graph::Graph, supported_nodes: Vec<(String, Box<fn() -> Node>)>){
    if supported_nodes.len() == 0
    {
        panic!("ERROR! Expected atleast 1 supported node");
    }
    eframe::run_native(
        app_name,
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(App{
            graph: graph,
            supported_new_nodes: supported_nodes,
            ..Default::default()
        })),
    )
    .unwrap();
}
