use anyhow::{anyhow, Result};
use std::io::prelude::*;
use std::{collections::HashMap, fs::File, path::Path};

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
    pub script_offset: (u16, u16),
    pub images: Vec<String>,
    pub current: usize,
    pub key_mapping: HashMap<char, String>,
    pub actions: Vec<Action>,
    pub output: String,
}

impl Default for App {
    fn default() -> Self {
        App {
            tab: 0,
            script_offset: (0, 0),
            images: vec![],
            current: 0,
            key_mapping: HashMap::new(),
            actions: vec![],
            output: "sort.sh".to_string(),
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
        self.tab = (self.tab + 1) % TABS.len();
        self.script_offset = (0, 0);
    }

    pub fn scroll_up(&mut self) {
        let (y, x) = self.script_offset;
        if y > 0 {
            self.script_offset = (y - 1, x);
        }
    }

    pub fn scroll_down(&mut self) {
        let (y, x) = self.script_offset;
        if y < self.actions.len() as u16 + 3 {
            self.script_offset = (y + 1, x);
        }
    }

    pub fn scroll_left(&mut self) {
        let (y, x) = self.script_offset;
        if x > 0 {
            self.script_offset = (y, x - 1);
        }
    }

    pub fn scroll_right(&mut self) {
        let (y, x) = self.script_offset;
        self.script_offset = (y, x + 1);
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

                        if !App::is_image(&path) {
                            continue;
                        }

                        let path_str = path.to_str();
                        if let Some(path_str) = path_str {
                            images.push(path_str.to_string());
                        }
                    }
                }
            }
        }

        self.images = images;
        Ok(())
    }

    pub fn write(&self) -> Result<()> {
        let mut lines: Vec<String> = vec!["#!/bin/sh".to_string()];

        for action in self.actions.iter() {
            match action {
                Action::MkDir(folder) => lines.push(format!("mkdir -p {}", folder)),
                Action::Move(image_path, folder) => {
                    lines.push(format!("mv {} {}", image_path, folder))
                }
                _ => {}
            }
        }

        let script = lines.join("\n");
        let mut file = File::create(&self.output)?;
        file.write_all(script.as_bytes())?;

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
