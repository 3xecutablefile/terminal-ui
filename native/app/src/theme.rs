#![allow(dead_code)]

use anyhow::{bail, Result};
use csscolorparser::Color as CssColor;
use dirs::config_dir;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Deserialize, Clone)]
pub struct Theme {
    pub meta: Meta,
    pub terminal: Terminal,
    pub ui: Ui,
    pub effects: Effects,
}

#[derive(Deserialize, Clone)]
pub struct Meta {
    pub name: String,
    pub author: String,
    pub variant: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct Terminal {
    pub foreground: String,
    pub background: String,
    pub cursor: String,
    pub selection: String,
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    pub white: String,
    pub bright_black: String,
    pub bright_red: String,
    pub bright_green: String,
    pub bright_yellow: String,
    pub bright_blue: String,
    pub bright_magenta: String,
    pub bright_cyan: String,
    pub bright_white: String,
}

#[derive(Deserialize, Clone)]
pub struct Ui {
    pub panel_bg: String,
    pub panel_border: String,
    pub text: String,
    pub accent: String,
}

#[derive(Deserialize, Clone)]
pub struct Effects {
    pub grid_color: String,
    pub grid_spacing: u32,
    pub scanline_opacity: f32,
    pub glow_intensity: f32,
}

pub fn load_theme(name: &str) -> Result<Theme> {
    let mut candidates = Vec::new();
    if let Some(mut dir) = config_dir() {
        dir.push("terminal-ui/themes");
        candidates.push(dir.join(format!("{name}.toml")));
    }
    candidates.push(PathBuf::from("native/app/assets/themes").join(format!("{name}.toml")));
    for path in candidates {
        if let Ok(data) = fs::read_to_string(&path) {
            let theme: Theme = toml::from_str(&data)?;
            return Ok(theme);
        }
    }
    bail!("theme '{name}' not found")
}

pub fn list_themes() -> Vec<String> {
    let mut names = Vec::new();
    if let Some(mut dir) = config_dir() {
        dir.push("terminal-ui/themes");
        if let Ok(rd) = fs::read_dir(dir) {
            for entry in rd.flatten() {
                if let Some(s) = entry.path().file_stem().and_then(|s| s.to_str()) {
                    names.push(s.to_string());
                }
            }
        }
    }
    if let Ok(rd) = fs::read_dir("native/app/assets/themes") {
        for entry in rd.flatten() {
            if let Some(s) = entry.path().file_stem().and_then(|s| s.to_str()) {
                if !names.contains(&s.to_string()) {
                    names.push(s.to_string());
                }
            }
        }
    }
    names
}

pub fn parse_color(s: &str) -> wgpu::Color {
    CssColor::from_str(s)
        .map(|c| wgpu::Color {
            r: c.r,
            g: c.g,
            b: c.b,
            a: c.a,
        })
        .unwrap_or(wgpu::Color::BLACK)
}
