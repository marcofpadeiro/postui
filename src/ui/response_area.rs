use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation};

use super::tabs::TuiTabs;

const TABS: [&str; 5] = ["Body", "Headers", "Cookies", "Scripts", "Trace"];

#[derive(Debug, Default)]
pub struct ResponseArea {
    pub content: String,
    pub tabs: TuiTabs,
}

impl ResponseArea {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            tabs: TuiTabs::new(TABS.iter().map(|s| s.to_string()).collect()),
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, is_focused: bool) {
        let block = Block::bordered()
            .title(" Response ")
            .title_alignment(Alignment::Right)
            .borders(Borders::ALL)
            .border_style(if is_focused {
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            });

        frame.render_widget(&block, area);

        let request_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2), Constraint::Min(0)])
            .split(block.inner(area));

        self.tabs.render(frame, request_area[0]);
        self.render_selected_tab(frame, request_area[1]);
    }

    fn render_selected_tab(&self, frame: &mut Frame, area: Rect) {
        // TODO: per tab content
        match self.tabs.selected {
            0 => self.render_body_tab(frame, area),
            _ => {
                let selected_tab = TABS[self.tabs.selected];
                let widget = Paragraph::new(selected_tab);

                frame.render_widget(widget, area);
            }
        }
    }

    fn render_body_tab(&self, frame: &mut Frame, area: Rect) {
        let widget = Paragraph::new(self.content.clone());
        frame.render_widget(widget, area);
    }

    pub fn on_key_event(&mut self, key: KeyEvent) {
        match key.code {
            _ => self.tabs.on_key_event(key),
        }
        return;
    }
}
