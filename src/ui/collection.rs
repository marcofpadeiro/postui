use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Scrollbar, ScrollbarOrientation};
use tui_tree_widget::{Tree, TreeItem, TreeState};

#[derive(Debug, Default)]
pub struct Collection {
    state: TreeState<&'static str>,
    pub items: Vec<TreeItem<'static, &'static str>>,
    pub is_expanded: bool,
}

impl Collection {
    pub fn new() -> Self {
        #[allow(unused)]
        Self {
            state: TreeState::default(),
            items: unsafe {
                vec![
                    TreeItem::new_leaf("0", "Request 0"),
                    TreeItem::new(
                        "1",
                        "Request 1",
                        vec![
                            TreeItem::new(
                                "1.1",
                                "Request 1.1",
                                vec![
                                    TreeItem::new_leaf("1.1.1", "Request 1.1.1"),
                                    TreeItem::new_leaf("1.1.2", "Request 1.1.2"),
                                ],
                            )
                            .expect("All unique item identifiers"),
                            TreeItem::new_leaf("1.2", "Request 1.2"),
                        ],
                    )
                    .expect("All unique item identifiers"),
                ]
            },
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
            )
            .highlight_symbol(">> ");

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
