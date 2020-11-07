use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum TabId {
    Main,
    Script,
}

const TABS: [TabId; 2] = [TabId::Main, TabId::Script];

pub struct App {
    pub images: Vec<String>,
    pub current: usize,
    pub key_bindings: HashMap<char, String>,
    pub tab: usize,
}

impl Default for App {
    fn default() -> Self {
        App {
            images: vec![],
            current: 0,
            key_bindings: HashMap::new(),
            tab: 0,
        }
    }
}

impl App {
    pub fn new() -> Self {
        let mut key_bindings = HashMap::new();
        key_bindings.insert('a', "./some-folder/a".to_string());
        key_bindings.insert('b', "./some-folder/b".to_string());
        key_bindings.insert('c', "./some-folder/c".to_string());
        key_bindings.insert('d', "./some-folder/d".to_string());
        key_bindings.insert('g', "./some-folder/g".to_string());

        App {
            images: vec!["/home/jgalat/git/image-sorter/rember.png".to_string()],
            key_bindings: key_bindings,
            ..App::default()
        }
    }

    pub fn current_image(&self) -> String {
        self.images[self.current].clone()
    }

    pub fn current_tab(&self) -> TabId {
        TABS[self.tab]
    }

    pub fn switch_tab(&mut self) {
        self.tab = (self.tab + 1) % TABS.len()
    }
}
