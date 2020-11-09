use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::path::Path;

#[derive(PartialEq, Clone, Copy)]
pub enum TabId {
    Main,
    Script,
}

const TABS: [TabId; 2] = [TabId::Main, TabId::Script];

#[derive(PartialEq)]
pub enum Action {
    Skip(String),
    Move(String, String),
    MkDir(String),
}

pub struct App {
    pub tab: usize,
    pub images: Vec<String>,
    pub current: usize,
    pub key_mapping: HashMap<char, String>,
    pub actions: Vec<Action>,
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
    pub fn current_image(&self) -> Option<String> {
        if self.current == self.images.len() {
            return None;
        }

        Some(self.images[self.current].clone())
    }

    pub fn pop_action(&mut self) {
        if self.current > 0 {
            self.actions.pop();
            self.current -= 1;
        }
    }

    pub fn push_action(&mut self, action: Action) {
        if self.current < self.images.len() {
            self.actions.push(action);
            self.current += 1;
        }
    }

    pub fn current_tab(&self) -> TabId {
        TABS[self.tab]
    }

    pub fn switch_tab(&mut self) {
        self.tab = (self.tab + 1) % TABS.len()
    }

    pub fn parse_key_mapping(&mut self, args: Vec<&str>) -> Result<()> {
        let mut key_mapping = HashMap::new();

        for i in (0..args.len() - 1).step_by(2) {
            let key = args[i].chars().next().unwrap();
            let path_str = args[i + 1].to_string();
            let path = Path::new(&path_str);

            if path.exists() && !path.is_dir() {
                return Err(anyhow!("{} exists and it's not a directory!", path_str));
            }

            if !path.exists() && path.to_str().is_some() {
                self.actions.push(Action::MkDir(path_str.clone()));
            }

            key_mapping.insert(key, path_str);
        }

        self.key_mapping = key_mapping;
        Ok(())
    }

    pub fn parse_input_files(&mut self, args: Vec<&str>) -> Result<()> {
        let mut images: Vec<String> = vec![];

        for input in args.iter() {
            let path = Path::new(input);

            if path.is_file() && App::is_image(&path) {
                images.push(input.to_string());
            }

            if path.is_dir() {
                let entries = path.read_dir()?;
                for entry in entries {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        let path_str = path.to_str();
                        if App::is_image(&path) && path_str.is_some() {
                            images.push(path_str.unwrap().to_string());
                        }
                    }
                }
            }
        }

        self.images = images;
        Ok(())
    }

    fn is_image(file: &Path) -> bool {
        let image_exts = ["jpeg", "jpg", "png"];

        for ext in image_exts.iter() {
            if let Some(file_ext) = file.extension() {
                if file_ext.to_str() == Some(ext) {
                    return true;
                }
            }
        }

        false
    }
}
