use std::collections::HashMap;

pub enum CurrentScreen {
    Home,
    Editor,
    Exit,
}

pub enum CurrentlyEditing {
    Key,
    Value,
}

pub struct App {
    pub key_input: String,
    pub value_input: String,
    pub pairs: HashMap<String, String>,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
}

impl App {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("name".to_string(), "zhang san".to_string());
        Self {
            key_input: String::new(),
            value_input: String::new(),
            pairs: map,
            current_screen: CurrentScreen::Home,
            currently_editing: None,
        }
    }

    pub fn save_key_value(&mut self) {
        self.pairs.insert(self.key_input.clone(), self.value_input.clone());
        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
    }

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Key => {
                    self.currently_editing = Some(CurrentlyEditing::Value);
                }
                CurrentlyEditing::Value => {
                    self.currently_editing = Some(CurrentlyEditing::Key);
                }
            }
        } else {
            self.currently_editing = Some(CurrentlyEditing::Key);
        }
    }

    pub fn print_json(&self) -> serde_json::Result<()> {
        let output_json = serde_json::to_string_pretty(&self.pairs)?;
        println!("{}", output_json);
        Ok(())
    }
}
