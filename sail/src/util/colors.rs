use itertools::Itertools;
use polars::prelude::*;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{self, Color, Modifier, Style, Stylize},
    text::Text,
    widgets::{
        Bar, BarChart, Block, BorderType, Borders, Cell, Clear, HighlightSpacing, Paragraph, Row,
        Scrollbar, ScrollbarOrientation, ScrollbarState, Table, TableState, Wrap,
    },
    DefaultTerminal, Frame,
};
use serde_json::Value;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use style::palette::tailwind;

const PALETTES: [tailwind::Palette; 4] = [
    tailwind::BLUE,
    tailwind::EMERALD,
    tailwind::INDIGO,
    tailwind::RED,
];

pub struct TableColors {
    pub buffer_bg: Color,
    pub header_bg: Color,
    pub header_fg: Color,
    pub row_fg: Color,
    pub selected_row_style_fg: Color,
    pub selected_column_style_fg: Color,
    pub selected_cell_style_fg: Color,
    pub normal_row_color: Color,
    pub alt_row_color: Color,
    pub footer_border_color: Color,
}

impl TableColors {
    const fn new(color: &tailwind::Palette) -> Self {
        Self {
            buffer_bg: tailwind::SLATE.c950,
            header_bg: color.c900,
            header_fg: tailwind::SLATE.c200,
            row_fg: tailwind::SLATE.c200,
            selected_row_style_fg: color.c400,
            selected_column_style_fg: color.c400,
            selected_cell_style_fg: color.c600,
            normal_row_color: tailwind::SLATE.c950,
            alt_row_color: tailwind::SLATE.c900,
            footer_border_color: color.c400,
        }
    }
    pub fn default_terminal() -> Self {
        Self {
            header_fg: Color::Reset,
            header_bg: Color::Reset,
            normal_row_color: Color::Reset,
            alt_row_color: Color::Reset,
            row_fg: Color::Reset,
            selected_row_style_fg: Color::Reset,
            selected_column_style_fg: Color::Reset,
            selected_cell_style_fg: Color::Reset,
            buffer_bg: Color::Reset,
            footer_border_color: Color::Reset,
        }
    }

    pub fn new_from_pywal() -> Self {
        // Get pywal colors or use default if not available
        if let Some(colors) = get_pywal_colors() {
            Self {
                buffer_bg: colors[0],
                header_bg: colors[1],
                header_fg: colors[7],
                row_fg: colors[7],
                selected_row_style_fg: colors[5],
                selected_column_style_fg: colors[5],
                selected_cell_style_fg: colors[6],
                normal_row_color: colors[0],
                alt_row_color: colors[8],
                footer_border_color: colors[4],
            }
        } else {
            // Fallback to default colors
            Self::new(&PALETTES[0])
        }
    }
}

// Add a function to read pywal colors
fn get_pywal_colors() -> Option<Vec<Color>> {
    // Get user's home directory using environment variables
    let home = env::var("HOME").ok()?;
    let cache_path = PathBuf::from(home).join(".cache/wal/colors.json");

    // Read the pywal colors.json file
    let mut file = match File::open(cache_path) {
        Ok(file) => file,
        Err(_) => return None,
    };

    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_err() {
        return None;
    }

    // Parse the JSON
    let json: Value = match serde_json::from_str(&contents) {
        Ok(json) => json,
        Err(_) => return None,
    };

    // Extract colors array
    let colors = match json.get("colors") {
        Some(colors) => colors,
        None => return None,
    };

    // Convert hex colors to ratatui Color objects
    let mut result = Vec::new();
    for i in 0..16 {
        let color_key = format!("color{}", i);
        if let Some(hex) = colors.get(&color_key).and_then(|v| v.as_str()) {
            // Remove the leading # if present
            let hex = hex.trim_start_matches('#');

            // Parse the hex color
            if let Ok(rgb) = u32::from_str_radix(hex, 16) {
                let r = ((rgb >> 16) & 0xFF) as u8;
                let g = ((rgb >> 8) & 0xFF) as u8;
                let b = (rgb & 0xFF) as u8;
                result.push(Color::Rgb(r, g, b));
            }
        }
    }

    Some(result)
}
