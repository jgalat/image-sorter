use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum TabId {
    Main,
    Script,
}

const TABS: [TabId; 2] = [TabId::Main, TabId::Script];

pub struct App {
    pub tab: usize,
    pub images: Vec<String>,
    pub current: usize,
    pub key_mapping: HashMap<char, String>,
    pub actions: Vec<(String, Option<String>)>,
}

impl Default for App {
    fn default() -> Self {
        App {
            tab: 0,
            images: vec![],
            current: 0,
            key_mapping: HashMap::new(),
            actions: vec![],
        }
    }
}

impl App {
    pub fn new() -> Self {
        let mut key_mapping = HashMap::new();
        key_mapping.insert('a', "./some-folder/a".to_string());
        key_mapping.insert('b', "./some-folder/b".to_string());
        key_mapping.insert('c', "./some-folder/c".to_string());
        key_mapping.insert('d', "./some-folder/d".to_string());
        key_mapping.insert('g', "./some-folder/g".to_string());

        App {
            images: vec![
                "./rember.png".to_string(),
                "./rember.png".to_string(),
                "./rember.png".to_string(),
                "./rember.png".to_string(),
                "./rember.png".to_string(),
            ],
            key_mapping: key_mapping,
            ..App::default()
        }
    }

    pub fn current_image(&self) -> String {
        self.images[self.current].clone()
    }

    pub fn revert_action(&mut self) {
        self.actions.pop();
        if self.current > 0 {
            self.current -= 1;
        }
    }

    pub fn add_action(&mut self, path: Option<String>) {
        let current_image = self.current_image();
        self.actions.push((current_image, path));
        if self.current < self.images.len() - 1 {
            self.current += 1;
        }
    }

    pub fn current_tab(&self) -> TabId {
        TABS[self.tab]
    }

    pub fn switch_tab(&mut self) {
        self.tab = (self.tab + 1) % TABS.len()
    }
}
