mod card;
mod common;
mod game_state;
mod info;

#[cfg(feature = "web")]
pub(crate) use game_state::render_game;
#[cfg(feature = "web")]
pub(crate) use info::{render_game_menu, render_game_over, render_help, render_start_screen};

use ratatui::buffer::Buffer;
use ratatui::style::Style;
#[cfg(feature = "native")]
use ratatui::DefaultTerminal;

#[cfg(feature = "native")]
pub struct Draw {
    terminal: DefaultTerminal,
    restored: bool,
}

pub(crate) struct Renderer<'a> {
    buf: &'a mut Buffer,
    style: Style,
    pub(super) cursor: usize,
    pub(super) debug_mode: bool,
}

impl<'a> Renderer<'a> {
    pub(crate) fn new(buf: &'a mut Buffer, cursor: usize, debug_mode: bool) -> Self {
        Self {
            buf,
            style: Style::default()
                .fg(Self::default_fg())
                .bg(Self::default_bg()),
            cursor,
            debug_mode,
        }
    }

    pub(crate) fn clear(&mut self) {
        let area = *self.buf.area();
        self.buf.set_style(
            area,
            Style::default().fg(Self::default_fg()).bg(Self::default_bg()),
        );
    }
}

#[cfg(feature = "native")]
impl Draw {
    pub fn new() -> Self {
        Self {
            terminal: ratatui::init(),
            restored: false,
        }
    }

    pub fn restore(&mut self) {
        if !self.restored {
            ratatui::restore();
            self.restored = true;
        }
    }
}

#[cfg(feature = "native")]
impl Drop for Draw {
    fn drop(&mut self) {
        self.restore();
    }
}
