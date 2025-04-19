use std::fs::read_dir;
use std::path::PathBuf;

use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Scrollbar, ScrollbarOrientation};
use tui_tree_widget::{Tree, TreeItem, TreeState};

use crate::config::get_requests_dir_path;

#[derive(Debug, Default)]
pub struct Collection {
    state: TreeState<String>,
    pub items: Vec<TreeItem<'static, String>>,
    pub is_expanded: bool,
}

impl Collection {
    pub fn new() -> Self {
        let collection_path = get_requests_dir_path().unwrap();
        Self {
            state: TreeState::default(),
            items: build_tree_from_dir(&collection_path),
            is_expanded: true,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, is_focused: bool) {
        let widget = Tree::new(&self.items)
            .expect("all item identifiers are unique")
            .block(
                Block::bordered()
                    .title(" Collection ")
                    .title_alignment(Alignment::Right)
                    .borders(Borders::ALL)
                    .border_style(if is_focused {
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    }),
            )
            .experimental_scrollbar(Some(
                Scrollbar::new(ScrollbarOrientation::VerticalRight)
                    .begin_symbol(None)
                    .track_symbol(None)
                    .end_symbol(None),
            ))
            .highlight_style(
                Style::new()
                    .fg(Color::Black)
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            );

        frame.render_stateful_widget(widget, area, &mut self.state);
    }

    pub fn on_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('\n' | ' ') => self.state.toggle_selected(),
            KeyCode::Left => self.state.key_left(),
            KeyCode::Right => self.state.key_right(),
            KeyCode::Down => self.state.key_down(),
            KeyCode::Up => self.state.key_up(),
            KeyCode::Esc => self.state.select(Vec::new()),
            KeyCode::Home => self.state.select_first(),
            KeyCode::End => self.state.select_last(),
            KeyCode::PageDown => self.state.scroll_down(3),
            KeyCode::PageUp => self.state.scroll_up(3),
            _ => false,
        };

        return;
    }
}

fn build_tree_from_dir(path: &PathBuf) -> Vec<TreeItem<'static, String>> {
    let mut items = Vec::new();

    if let Ok(entries) = read_dir(path) {
        entries.flatten().into_iter().for_each(|entry| {
            if let Ok(file_name) = entry.file_name().into_string() {
                let entry_path = entry.path();
                let entry_path_str = entry_path.to_string_lossy().to_string();

                if entry_path.is_dir() {
                    let children = build_tree_from_dir(&entry_path);
                    if let Ok(branch) = TreeItem::new(entry_path_str, file_name, children) {
                        items.push(branch);
                    }

                    return;
                }

                if entry_path.extension() != Some("toml".as_ref()) {
                    return;
                }

                items.push(TreeItem::new_leaf(entry_path_str, file_name));
            }
        });
    }

    items
}
