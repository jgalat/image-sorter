use anyhow::{anyhow, Result};
use std::io::prelude::*;
use std::{
    collections::HashMap,
    fs::File,
    path::{Path, PathBuf},
};

use crate::Opt;

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

impl App {
    pub fn new(opt: Opt) -> Result<Self> {
        let images = App::parse_images(opt.input)?;
        let (key_mapping, actions) = App::parse_key_mapping(opt.bind)?;

        Ok(App {
            tab: 0,
            script_offset: (0, 0),
            images: images,
            current: 0,
            key_mapping: key_mapping,
            actions: actions,
            output: opt.output,
        })
    }

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

    pub fn parse_key_mapping(
        args: Vec<(char, PathBuf)>,
    ) -> Result<(HashMap<char, String>, Vec<Action>)> {
        let mut key_mapping = HashMap::new();
        let mut actions = vec![];

        for (key, path) in args.into_iter() {
            let path = path.as_path();
            let path_string = path.display().to_string();

            if path.exists() && !path.is_dir() {
                return Err(anyhow!(
                    "{} exists and it's not a directory!",
                    path.display()
                ));
            }

            if !path.exists() {
                actions.push(Action::MkDir(path_string.clone()));
            }

            key_mapping.insert(key, path_string);
        }

        Ok((key_mapping, actions))
    }

    pub fn parse_images(args: Vec<PathBuf>) -> Result<Vec<String>> {
        let mut images: Vec<String> = vec![];

        for input in args.iter() {
            let path = input.as_path();

            if path.is_file() && App::is_image(&path) {
                images.push(path.display().to_string());
            }

            if path.is_dir() {
                for entry in path.read_dir()? {
                    if let Ok(entry) = entry {
                        let path = entry.path();

                        if !App::is_image(&path) {
                            continue;
                        }

                        let path_str = path.display().to_string();
                        images.push(path_str);
                    }
                }
            }
        }

        Ok(images)
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
