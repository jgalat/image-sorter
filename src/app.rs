use anyhow::{anyhow, Result};
use std::io::prelude::*;
use std::{
    collections::BTreeMap,
    fs::File,
    path::{Path, PathBuf},
    time::Instant,
};

use crate::Opt;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum TabId {
    Main,
    Script,
}

const TABS: [TabId; 2] = [TabId::Main, TabId::Script];

#[derive(PartialEq, Eq, Clone)]
pub enum Action {
    Skip(PathBuf),
    Move(PathBuf, PathBuf),
    Rename(String),
    MkDir(PathBuf),
    Delete(PathBuf),
}

impl Action {
    pub fn is_poppable(&self) -> bool {
        !matches!(self, Action::MkDir(_))
    }

    pub fn queue_step(&self) -> usize {
        match self {
            Action::Skip(_) | Action::Move(_, _) | Action::Delete(_) => 1,
            Action::Rename(_) | Action::MkDir(_) => 0,
        }
    }
}

pub struct App {
    pub tab: usize,
    pub script_offset: (u16, u16),
    pub images: Vec<PathBuf>,
    pub current: usize,
    pub key_mapping: BTreeMap<char, PathBuf>,
    pub actions: Vec<Action>,
    pub output: String,
    pub enable_input: bool,
    pub input: Vec<char>,
    pub input_idx: usize,
    pub last_save: Option<Instant>,
}

impl Default for App {
    fn default() -> Self {
        App {
            tab: 0,
            script_offset: (0, 0),
            current: 0,
            images: vec![],
            key_mapping: BTreeMap::new(),
            actions: vec![],
            output: "".to_string(),
            enable_input: false,
            input: vec![],
            input_idx: 0,
            last_save: None,
        }
    }
}

impl App {
    pub fn new(opt: Opt) -> Result<Self> {
        let images = App::parse_images(opt.input, opt.recurse);
        let (key_mapping, actions) = App::parse_key_mapping(opt.bind)?;

        Ok(App {
            images,
            key_mapping,
            actions,
            output: opt.output,
            ..App::default()
        })
    }

    pub fn current_image(&self) -> Option<PathBuf> {
        if self.current == self.images.len() {
            return None;
        }

        Some(self.images[self.current].clone())
    }

    pub fn pop_action(&mut self) {
        let last_action = self.actions.last().cloned();

        if let Some(last_action) = last_action {
            if last_action.is_poppable() {
                self.actions.pop();
            }
            self.current -= last_action.queue_step();
        }
    }

    pub fn push_action(&mut self, action: Action) {
        if self.current == self.images.len() {
            return;
        }

        self.current += action.queue_step();
        self.actions.push(action);
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

    pub fn rename_current_image(&mut self) {
        if let Some(current_image) = self.current_image() {
            if let Some(name) = current_image.file_name() {
                let name: Vec<char> = name.to_str().unwrap().chars().collect();
                self.input_idx = name.len();
                self.input = name;
                self.enable_input = true;
            }
        }
    }

    pub fn write(&mut self) -> Result<()> {
        let mut lines: Vec<String> = vec!["#!/bin/sh".to_string()];

        for action in self.actions.iter() {
            match action {
                Action::MkDir(folder) => lines.push(format!("mkdir -p \"{}\"", folder.display())),
                Action::Move(image_path, folder) => lines.push(format!(
                    "mv \"{}\" \"{}\"",
                    image_path.display(),
                    folder.display()
                )),
                Action::Delete(image) => lines.push(format!("rm \"{}\"", image.display())),
                _ => {}
            }
        }

        let script = lines.join("\n");
        let mut file = File::create(&self.output)?;
        file.write_all(script.as_bytes())?;

        self.last_save = Some(Instant::now());
        Ok(())
    }

    pub fn parse_key_mapping(
        args: Vec<(char, PathBuf)>,
    ) -> Result<(BTreeMap<char, PathBuf>, Vec<Action>)> {
        let mut key_mapping = BTreeMap::new();
        let mut actions = vec![];

        for (key, path_buf) in args.into_iter() {
            let path = path_buf.as_path();

            if path.exists() && !path.is_dir() {
                return Err(anyhow!(
                    "{} exists and it's not a directory!",
                    path.display()
                ));
            }

            if !path.exists() {
                actions.push(Action::MkDir(path_buf.clone()));
            }

            key_mapping.insert(key, path_buf);
        }

        Ok((key_mapping, actions))
    }

    pub fn parse_images(args: Vec<PathBuf>, recurse: bool) -> Vec<PathBuf> {
        let mut images: Vec<PathBuf> = vec![];

        let mut count = 0u32;

        for input in args {
            if App::is_image(&input) {
                count += 1;
                images.push(input);
            } else if input.is_dir() {
                images.extend(App::discover_images(&input, recurse, &mut count));
            }
        }

        images
    }

    fn discover_images(path: &Path, recurse: bool, count: &mut u32) -> Vec<PathBuf> {
        let mut images = vec![];

        if !path.is_dir() {
            return images;
        }

        let entries = match path.read_dir() {
            Ok(entries) => entries,
            Err(_) => return images,
        };

        for entry in entries.flatten() {
            let path = entry.path();

            if *count >= 500 {
                // TODO: Add an option for this value
                // Limit the number of images, to halt a potential runaway
                // program. The user will probably appreciate working with fewer
                // images, but at least being able to start the program.
                //
                // After having run the output script, the next invocation would
                // handle the following 500 images,
                break;
            }

            if path.is_dir() && recurse {
                images.extend(App::discover_images(&path, recurse, count));
            } else if App::is_image(&path) {
                *count += 1;
                images.push(path);
            }
        }

        images
    }

    fn is_image(path: &Path) -> bool {
        if !path.is_file() {
            return false;
        }

        // first, a quick check for the file extension
        let image_exts = ["jpeg", "jpg", "png"];
        let looks_like_image = path.extension().map_or(false, |f| {
            image_exts.iter().any(|ext| f.to_str() == Some(ext))
        });
        if !looks_like_image {
            return false;
        }

        // second, check the file's mime type by reading the first few bytes
        let kind = infer::get_from_path(path);
        if kind.is_err() {
            // could not read file
            return false;
        }
        let kind = kind.unwrap();
        if kind.is_none() {
            // unknown file type
            return false;
        }

        let kind = kind.unwrap();
        matches!(kind.mime_type(), "image/jpeg" | "image/png")
    }
}
