use ratatui::crossterm::event::KeyEvent;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, Tabs};

use super::tabs::TuiTabs;

const TABS: [&str; 7] = [
    "Headers", "Body", "Query", "Auth", "Info", "Scripts", "Options",
];

#[derive(Debug, Default)]
pub struct RequestArea {
    // TODO: Change to have request data
    pub content: String,
    pub tabs: TuiTabs,
}

impl RequestArea {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            tabs: TuiTabs::new(TABS.iter().map(|s| s.to_string()).collect()),
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, is_focused: bool) {
        let block = Block::bordered()
            .title(" Request ")
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
        let selected_tab = TABS[self.tabs.selected];
        let widget = Paragraph::new(selected_tab);

        frame.render_widget(widget, area);
    }

    pub fn on_key_event(&mut self, key: KeyEvent) {
        match key.code {
            // tab handler
            _ => self.tabs.on_key_event(key),
        }
        return;
    }
}
