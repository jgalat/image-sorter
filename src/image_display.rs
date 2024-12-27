use anyhow::{anyhow, Result};
use ratatui::layout::Rect;
use std::{
    env,
    path::{Path, PathBuf},
};
use subprocess::{Popen, PopenConfig, Redirection};

pub struct ImageDisplay {
    path: String,
}

impl ImageDisplay {
    pub fn new() -> Result<Self> {
        let mut paths = vec![
            "/usr/lib/w3m/w3mimgdisplay",
            "/usr/libexec/w3m/w3mimgdisplay",
            "/usr/lib64/w3m/w3mimgdisplay",
            "/usr/libexec64/w3m/w3mimgdisplay",
            "/usr/local/libexec/w3m/w3mimgdisplay",
        ];

        let env_path = env::var("W3MIMGDISPLAY_PATH").unwrap_or_else(|_| "".to_string());
        if !env_path.is_empty() {
            paths.insert(0, &env_path);
        }

        let mut w3m_path: Option<&str> = None;

        for path in paths {
            if Path::new(path).exists() {
                w3m_path = Some(path)
            }
        }

        if let Some(path) = w3m_path {
            Ok(ImageDisplay {
                path: path.to_string(),
            })
        } else {
            Err(anyhow!("w3mimgdisplay is not available!"))
        }
    }

    pub fn render_image(&self, image_path: PathBuf, block: Rect, terminal: Rect) -> Result<()> {
        let input = self.w3m_input(image_path, block, terminal)?;
        let mut process = Popen::create(
            &[&self.path],
            PopenConfig {
                stdin: Redirection::Pipe,
                stdout: Redirection::Pipe,
                ..PopenConfig::default()
            },
        )?;
        process.communicate(Some(&input))?;
        process.kill()?;
        Ok(())
    }

    fn w3m_input(&self, image_path: PathBuf, block: Rect, terminal: Rect) -> Result<String> {
        let (fontw, fonth) = self.font_dimensions(terminal)?;

        let start_x = (block.x as u32 + 1) * fontw;
        let start_y = (block.y as u32 + 1) * fonth;

        let max_width = (block.width as u32 - 1) * fontw;
        let max_height = (block.height as u32 - 1) * fonth;

        let (mut width, mut height) = self.image_dimensions(&image_path)?;
        if width > max_width {
            // width _ height
            // max_width _ max_width * height / width
            height = max_width * height / width;
            width = max_width;
        }
        if height > max_height {
            // height _ width
            // max_height _ max_height * width / height
            width = max_height * width / height;
            height = max_height;
        }

        let input = format!(
            "0;1;{};{};{};{};;;;;{}\n4;\n3;\n",
            start_x,
            start_y,
            width,
            height,
            image_path.display()
        );

        Ok(input)
    }

    fn image_dimensions(&self, image_path: &Path) -> Result<(u32, u32)> {
        let input = format!("5;{}\n", image_path.display());
        let mut process = Popen::create(
            &[&self.path],
            PopenConfig {
                stdin: Redirection::Pipe,
                stdout: Redirection::Pipe,
                ..PopenConfig::default()
            },
        )?;
        let (out, err) = process.communicate(Some(&input))?;
        process.kill()?;
        if err.is_some() || out.is_none() {
            return Err(anyhow!("w3mimagedisplay failed image dimensions"));
        }

        let outputs = out.unwrap();
        let outputs = outputs.trim().split(' ').collect::<Vec<&str>>();
        if outputs.len() < 2 {
            return Err(anyhow!(
                "w3mimagedisplay wrong output (image dimensions) for input file {}",
                image_path.display()
            ));
        }

        let width = outputs[0].parse::<u32>()?;
        let height = outputs[1].parse::<u32>()?;

        Ok((width, height))
    }

    fn font_dimensions(&self, terminal: Rect) -> Result<(u32, u32)> {
        let path = self.path.clone();
        let mut process = Popen::create(
            &[path, "-test".to_string()],
            PopenConfig {
                stdout: Redirection::Pipe,
                ..PopenConfig::default()
            },
        )?;

        let (out, err) = process.communicate(None)?;
        process.kill()?;
        if err.is_some() || out.is_none() {
            return Err(anyhow!("w3mimagedisplay failed -test"));
        }

        let outputs = out.unwrap();
        let outputs = outputs.trim().split(' ').collect::<Vec<&str>>();

        if outputs.len() < 2 {
            return Err(anyhow!("w3mimagedisplay wrong output (font dimensions)"));
        }

        let xwidth = outputs[0].parse::<u32>()? + 2;
        let xheight = outputs[1].parse::<u32>()? + 2;

        Ok((
            xwidth / terminal.width as u32,
            xheight / terminal.height as u32,
        ))
    }
}
