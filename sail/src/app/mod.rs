// Polars imports
use polars::prelude::*;

// Ratatui imports
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    prelude::*,
    style::{Modifier, Style},
    text::Text,
    widgets::{
        Block, BorderType, Borders, Cell, Clear, HighlightSpacing, Paragraph, Row, Scrollbar,
        ScrollbarOrientation, ScrollbarState, Table, TableState, Wrap,
    },
    DefaultTerminal, Frame,
};

// Your internal module imports
use crate::data::get_data_frame;
use crate::util::colors::TableColors;

const INFO_TEXT: [&str; 2] = [
    "(Esc) quit | (k) move up | (j) move down | (h) move left | (l) move right",
    "(Shift + →) next color | (Shift + ←) previous color",
];

const ITEM_HEIGHT: usize = 4;

pub struct App {
    state: TableState,
    df: DataFrame,
    column_widths: Vec<u16>,
    scroll_state: ScrollbarState,
    colors: TableColors,
    showing_summary: bool,
}

impl App {
    pub fn new(file_path: &str) -> Self {
        // Create DataFrame instead of vector of Data structs
        let df = get_data_frame(&file_path).expect("Failed to load CSV file");

        // Calculate constraints based on DataFrame
        let column_widths = App::constraint_len_calculator(&df);

        let height = df.height();

        // Start at col 0
        let mut state = TableState::default().with_selected(0);
        state.select_next_column();

        Self {
            state,
            df,
            column_widths,
            scroll_state: ScrollbarState::new((height - 1) * ITEM_HEIGHT),
            colors: TableColors::new_from_pywal(),
            showing_summary: false,
        }
    }

    fn next_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.df.height() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn previous_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.df.height() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn next_column(&mut self) {
        self.state.select_next_column();
    }

    pub fn previous_column(&mut self) {
        self.state.select_previous_column();
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    let shift_pressed = key.modifiers.contains(KeyModifiers::SHIFT);
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Char('j') | KeyCode::Down => self.next_row(),
                        KeyCode::Char('k') | KeyCode::Up => self.previous_row(),
                        KeyCode::Char('l') | KeyCode::Right => self.next_column(),
                        KeyCode::Char('h') | KeyCode::Left => self.previous_column(),
                        KeyCode::Char(' ') => {
                            self.showing_summary = !self.showing_summary;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let vertical = &Layout::vertical([Constraint::Min(5), Constraint::Length(4)]);
        let rects = vertical.split(frame.size());
        self.render_table(frame, rects[0]);
        self.render_scrollbar(frame, rects[0]);
        self.render_footer(frame, rects[1]);

        if self.showing_summary {
            self.render_summary_popup(frame);
        }
    }
    fn render_summary_popup(&self, frame: &mut Frame) {
        // Create a rect for the right half of the screen
        let area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(frame.area())[1]; // Use the right half

        // Clears anything under the popup so it draws cleanly
        frame.render_widget(Clear, area);

        // Get the selected column
        let selected_col = self.state.selected_column().unwrap();
        let col_name = self.df.get_column_names()[selected_col];
        let col = self.df.column(col_name).unwrap();

        // Split the popup area into sections
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),      // Title
                Constraint::Percentage(40), // Stats
                Constraint::Percentage(60), // Graph
            ])
            .split(area);

        // Title
        let title = Paragraph::new(format!("Column: {}", col_name))
            .block(Block::default().borders(Borders::ALL).title("Summary"))
            .alignment(Alignment::Center);
        frame.render_widget(title, popup_layout[0]);

        // Stats section
        // Generate a distribution of the current column

        let stats_text = format!(
            "Mean: {}\nMedian: {}\nStd Dev: {}\nMin: {:?}\nMax: {:?}",
            match col.mean() {
                Some(val) => val.to_string(),
                None => "N/A".to_string(),
            },
            match col.median() {
                Some(val) => val.to_string(),
                None => "N/A".to_string(),
            },
            match col.std(1) {
                Some(val) => val.to_string(),
                None => "N/A".to_string(),
            },
            match col.min::<f64>() {
                Ok(val) => match val {
                        Some(val) => val.to_string(),
                        None => "N/A".to_string(),
                    }
                },
                Err(_) => "N/A".to_string(),
            },
            match col.max::<f64>() {
                Ok(val) => val.unwrap().to_string(),
                Err(_) => "N/A".to_string(),
            }
        );

        let stats = Paragraph::new(stats_text)
            .block(Block::default().borders(Borders::ALL).title("Statistics"))
            .wrap(Wrap { trim: true });
        frame.render_widget(stats, popup_layout[1]);
    }

    fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(vertical_chunks[1])[1]
    }
    fn render_table(&mut self, frame: &mut Frame, area: Rect) {
        let header_style = Style::default()
            .fg(self.colors.header_fg)
            .bg(self.colors.header_bg);

        let selected_row_style = Style::default().fg(self.colors.selected_row_style_fg);

        let selected_col_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.colors.selected_column_style_fg);
        let selected_cell_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.colors.selected_cell_style_fg);

        // Get column names from DataFrame
        let header = self
            .df
            .get_column_names()
            .iter()
            .map(|name| Cell::from(*name))
            .collect::<Row>()
            .style(header_style)
            .height(1);

        // Create constraints vector for all columns
        let constraints: Vec<Constraint> = self
            .column_widths
            .iter()
            .enumerate()
            .map(|(i, &width)| {
                if i == 0 {
                    // First column has fixed width
                    Constraint::Length(width + 1)
                } else {
                    // Other columns have minimum width
                    Constraint::Min(width + 1)
                }
            })
            .collect();

        let rows = (0..self.df.height()).map(|i| {
            let color = if i % 2 == 0 {
                self.colors.normal_row_color
            } else {
                self.colors.alt_row_color
            };

            let item = self.get_row_as_strings(i);
            item.into_iter()
                .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
                .collect::<Row>()
                .style(Style::new().fg(self.colors.row_fg).bg(color))
                .height(4)
        });

        let bar = " █ ";
        let t = Table::new(
            rows,
            constraints, // Use the vector of constraints
        )
        .header(header)
        .row_highlight_style(selected_row_style)
        .column_highlight_style(selected_col_style)
        .cell_highlight_style(selected_cell_style)
        .highlight_symbol(Text::from(vec![
            "".into(),
            bar.into(),
            bar.into(),
            "".into(),
        ]))
        .bg(self.colors.buffer_bg)
        .highlight_spacing(HighlightSpacing::Always);

        frame.render_stateful_widget(t, area, &mut self.state);
    }
    fn render_scrollbar(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None),
            area.inner(Margin {
                vertical: 1,
                horizontal: 1,
            }),
            &mut self.scroll_state,
        );
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let info_footer = Paragraph::new(Text::from_iter(INFO_TEXT))
            .style(
                Style::new()
                    .fg(self.colors.row_fg)
                    .bg(self.colors.buffer_bg),
            )
            .centered()
            .block(
                Block::bordered()
                    .border_type(BorderType::Double)
                    .border_style(Style::new().fg(self.colors.footer_border_color)),
            );
        frame.render_widget(info_footer, area);
    }
    fn get_row_as_strings(&self, i: usize) -> Vec<String> {
        self.df
            .get_columns()
            .iter()
            .map(|col| {
                col.get(i)
                    .unwrap_or(polars::prelude::AnyValue::Null)
                    .to_string()
            })
            .collect()
    }

    fn constraint_len_calculator(df: &DataFrame) -> Vec<u16> {
        // Get column names
        let column_names = df.get_column_names();

        // Calculate max width for each column
        column_names
            .iter()
            .map(|&col_name| {
                let width = df
                    .column(col_name)
                    .unwrap()
                    .cast(&DataType::String)
                    .unwrap()
                    .str()
                    .unwrap()
                    .into_iter()
                    .filter_map(|opt_s| opt_s.map(|s| unicode_width::UnicodeWidthStr::width(s)))
                    .max()
                    .unwrap_or(10);

                // Add some padding and ensure minimum width
                let col_width = width.max(col_name.len()) + 2;

                #[allow(clippy::cast_possible_truncation)]
                (col_width as u16)
            })
            .collect()
    }
}
