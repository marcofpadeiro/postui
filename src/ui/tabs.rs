use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::*;
use ratatui::widgets::Tabs;

#[derive(Debug, Default)]
pub struct TuiTabs {
    tabs: Vec<String>,
    pub selected: usize,
}

impl TuiTabs {
    pub fn new(tabs: Vec<String>) -> Self {
        Self { tabs, selected: 0 }
    }

    fn next(&mut self) {
        self.selected = (self.selected + 1) % self.tabs.len();
    }

    fn previous(&mut self) {
        if self.selected == 0 {
            self.selected = self.tabs.len() - 1;
            return;
        }

        self.selected -= 1;
    }

    pub fn go_to(&mut self, index: usize) {
        if index > self.tabs.len() {
            self.selected = 0;
        }

        self.selected = index;
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let highlight_style = Style::default()
            .fg(Color::White)
            .underlined()
            .add_modifier(Modifier::BOLD);

        let selected_tab_index = self.selected as usize;
        let widget = Tabs::new(self.tabs.clone())
            .highlight_style(highlight_style)
            .select(selected_tab_index)
            .padding(" ", " ")
            .divider(" ");

        frame.render_widget(widget, area);
    }

    pub fn on_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Left | KeyCode::Char('h') => {
                self.previous();
            }
            KeyCode::Right | KeyCode::Char('l') => {
                self.next();
            }
            _ => {}
        }
        return;
    }
}
