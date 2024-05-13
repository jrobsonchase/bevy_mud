use std::fmt::Display;

use bevy::prelude::{
  Deref,
  DerefMut,
};
use ratatui::{
  prelude::*,
  widgets::Widget,
};

#[inline]
fn to_ansi(tui: Style) -> ansi_term::Style {
  ansi_term::Style {
    foreground: tui.fg.and_then(to_ansi_color),
    background: tui.bg.and_then(to_ansi_color),

    is_bold: tui.add_modifier.contains(Modifier::BOLD),
    is_dimmed: tui.add_modifier.contains(Modifier::DIM),
    is_italic: tui.add_modifier.contains(Modifier::ITALIC),
    is_underline: tui.add_modifier.contains(Modifier::UNDERLINED),
    is_blink: tui.add_modifier.contains(Modifier::SLOW_BLINK)
      || tui.add_modifier.contains(Modifier::RAPID_BLINK),
    is_reverse: tui.add_modifier.contains(Modifier::REVERSED),
    is_hidden: tui.add_modifier.contains(Modifier::HIDDEN),
    is_strikethrough: tui.add_modifier.contains(Modifier::CROSSED_OUT),
  }
}

#[inline]
fn to_ansi_color(tui: Color) -> Option<ansi_term::Colour> {
  Some(match tui {
    Color::Reset => return None,
    Color::Black => ansi_term::Colour::Black,
    Color::Red => ansi_term::Colour::Red,
    Color::Green => ansi_term::Colour::Green,
    Color::Yellow => ansi_term::Colour::Yellow,
    Color::Blue => ansi_term::Colour::Blue,
    Color::Magenta => ansi_term::Colour::Purple,
    Color::Cyan => ansi_term::Colour::Cyan,
    Color::Gray => ansi_term::Colour::White,
    Color::DarkGray => ansi_term::Colour::Fixed(8),
    Color::LightRed => ansi_term::Colour::Fixed(9),
    Color::LightGreen => ansi_term::Colour::Fixed(10),
    Color::LightYellow => ansi_term::Colour::Fixed(11),
    Color::LightBlue => ansi_term::Colour::Fixed(12),
    Color::LightMagenta => ansi_term::Colour::Fixed(13),
    Color::LightCyan => ansi_term::Colour::Fixed(14),
    Color::White => ansi_term::Colour::Fixed(15),
    Color::Indexed(i) => ansi_term::Colour::Fixed(i),
    Color::Rgb(r, g, b) => ansi_term::Colour::RGB(r, g, b),
  })
}

#[derive(Debug, Default, Deref, DerefMut)]
pub struct Ansi(pub Buffer);

impl Ansi {
  #[allow(dead_code)]
  pub fn render_widget(&mut self, widget: impl Widget, area: Rect) {
    widget.render(area, &mut *self)
  }
}

impl Display for Ansi {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let area = self.area();
    let mut prev_style = ansi_term::Style::default();
    for y in area.y..area.y + area.height {
      for x in area.x..area.x + area.width {
        let cell = self.get(x, y);
        let style = to_ansi(cell.style());
        let sym = &cell.symbol();
        if style != prev_style {
          write!(f, "{}", prev_style.infix(style)).unwrap();
        }
        write!(f, "{sym}")?;
        prev_style = style;
      }
      writeln!(f, "{}", prev_style.infix(ansi_term::Style::default()))?;
      prev_style = ansi_term::Style::default();
    }
    Ok(())
  }
}
