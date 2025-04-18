use std::error::Error;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::prelude::*;
use ratatui::widgets::Borders;
use ratatui::{DefaultTerminal, Frame, widgets::Block};

use super::area::Area;

#[derive(Debug, Default)]
#[allow(unused)]
pub struct Tui {
    collection_expanded: bool,
    focus: Area,
    editing: bool,
    running: bool,
}

impl Tui {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<(), Box<dyn Error>> {
        self.running = true;

        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let url_bottom = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(5), Constraint::Min(0)])
            .split(frame.area());

        let col_req_constraints = if self.collection_expanded {
            [Constraint::Length(30), Constraint::Min(0)]
        } else {
            [Constraint::Length(0), Constraint::Min(0)]
        };

        let col_req_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(col_req_constraints)
            .split(url_bottom[1]);

        let header_body_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(col_req_area[1]);

        frame.render_widget(
            Block::default()
                .title(" Postui ")
                .title_alignment(Alignment::Right)
                .style(if self.focus == Area::Url {
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Gray)
                }),
            url_bottom[0],
        );

        self.render_layout_block(
            frame,
            col_req_area[0],
            " Collection ",
            self.focus == Area::Collection,
        );
        self.render_layout_block(
            frame,
            header_body_area[0],
            " Request ",
            self.focus == Area::Request,
        );
        self.render_layout_block(
            frame,
            header_body_area[1],
            " Response ",
            self.focus == Area::Body,
        );
    }

    fn render_layout_block(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        title: &str,
        is_focused: bool,
    ) {
        let block = Block::default()
            .title(title)
            .title_alignment(Alignment::Right)
            .borders(Borders::ALL)
            .border_style(if is_focused {
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            });

        frame.render_widget(block, area);
    }

    fn handle_crossterm_events(&mut self) -> Result<(), Box<dyn Error>> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        if self.editing {
            if key.code == KeyCode::Esc {
                self.editing = false;
            }
            return;
        }

        match (key.modifiers, key.code) {
            (_, KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            (_, KeyCode::Char('e')) => {
                self.collection_expanded = !self.collection_expanded;
            }
            (_, KeyCode::Char('j')) => {
                self.focus = self.focus.next(self.collection_expanded);
            }
            (_, KeyCode::Char('k')) => {
                self.focus = self.focus.previous(self.collection_expanded);
            }
            (_, KeyCode::Char('i')) => {
                self.editing = true;
            }
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
