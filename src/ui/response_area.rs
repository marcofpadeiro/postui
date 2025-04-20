use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation};


#[derive(Debug, Default)]
pub struct ResponseArea {
    pub content: String,
}

impl ResponseArea {
    pub fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, is_focused: bool) {
        let widget = Paragraph::new(self.content.clone())
            .block(
                Block::bordered()
                    .title(" Response ")
                    .title_alignment(Alignment::Right)
                    .borders(Borders::ALL)
                    .border_style(if is_focused {
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    }),
            );

        frame.render_widget(widget, area);
    }

    pub fn on_key_event(&mut self, key: KeyEvent) {
        return;
    }
}
