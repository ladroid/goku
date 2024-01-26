use crate::gui::component::Component;
use crate::gui::state::State;

pub struct DisplayComponentTree<'a> {
    pub ui: &'a imgui::Ui,
    pub state: &'a mut State,
}

impl<'a> DisplayComponentTree<'a> {
    pub fn display(&mut self, components: &[Component], level: usize) {
        for component in components {
            // Construct the label for the tree node, including the indentation
            let label = format!("{:indent$}{}", "", component.name, indent = level * 2);
    
            // Create a tree node for each component
            if let Some(node) = self.ui.tree_node(&imgui::ImString::new(label)) {
                // If the node is clicked, update the selected component
                if self.ui.is_item_clicked() {
                    self.state.selected_component = Some(component.name.clone());
                }
    
                // Recursively display child components, increasing the level for indentation
                self.display(&component.children, level + 1);
    
                // End the tree node
                node.end();
            }
        }
    }    
}