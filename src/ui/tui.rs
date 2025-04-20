use std::error::Error;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;
use ratatui::{DefaultTerminal, Frame, widgets::Block};

use crate::request::executor::perform_request;
use crate::request::model::Request;
use crate::request::parser::parse_file;

use super::area::Area;
use super::collection::Collection;
use super::request_area::RequestArea;
use super::response_area::ResponseArea;

#[derive(Debug, Default)]
pub struct Tui {
    request: Request,
    collection: Collection,
    response_area: ResponseArea,
    request_area: RequestArea,
    focus: Area,
    editing: bool,
    running: bool,
}

impl Tui {
    pub fn new() -> Self {
        Self {
            request: Request::default(),
            collection: Collection::new(),
            response_area: ResponseArea::new(),
            request_area: RequestArea::new(),
            focus: Area::Url,
            editing: false,
            running: false,
        }
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<(), Box<dyn Error>> {
        self.running = true;

        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events().await?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let url_bottom = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(5), Constraint::Min(0)])
            .split(frame.area());

        let col_req_constraints = if self.collection.is_expanded {
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
            Paragraph::new(self.request.url.to_raw())
                .block(
                    Block::default()
                        .title(" Postui ")
                        .title_alignment(Alignment::Right),
                )
                .style(Style::default().fg(Color::Gray)),
            url_bottom[0],
        );

        self.collection
            .render(frame, col_req_area[0], self.focus == Area::Collection);

        self.response_area
            .render(frame, header_body_area[1], self.focus == Area::Response);

        self.request_area
            .render(frame, header_body_area[0], self.focus == Area::Request);
    }

    async fn handle_crossterm_events(&mut self) -> Result<(), Box<dyn Error>> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key).await,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    async fn on_key_event(&mut self, key: KeyEvent) {
        if self.editing {
            if key.code == KeyCode::Esc {
                self.editing = false;
            }
            return;
        }

        match (key.modifiers, key.code) {
            // Generic app controls
            (_, KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            (KeyModifiers::CONTROL, KeyCode::Char('e')) => {
                if self.focus == Area::Collection {
                    self.focus = Area::Url;
                }
                self.collection.is_expanded = !self.collection.is_expanded;
            }
            (KeyModifiers::CONTROL, KeyCode::Char('j')) => {
                self.focus = self.focus.next(self.collection.is_expanded);
            }
            (KeyModifiers::CONTROL, KeyCode::Char('k')) => {
                self.focus = self.focus.previous(self.collection.is_expanded);
            }
            (KeyModifiers::CONTROL, KeyCode::Char('r')) => {
                let response = perform_request(self.request.clone()).await.unwrap();
                self.response_area.content = response.parse_response().await;
                self.response_area.tabs.go_to(0);
            }
            (_, _) => {
                // Handle other areas
                match self.focus {
                    Area::Collection => {
                        if let Some(selected) = self.collection.on_key_event(key) {
                            self.request = parse_file(&selected).unwrap();
                        }
                    }
                    Area::Url => {}
                    Area::Request => self.request_area.on_key_event(key),
                    Area::Response => self.response_area.on_key_event(key),
                }
            }
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
