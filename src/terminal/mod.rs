//  notty is a new kind of terminal emulator.
//  Copyright (C) 2015 without boats
//  
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU Affero General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//  
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU Affero General Public License for more details.
//  
//  You should have received a copy of the GNU Affero General Public License
//  along with this program.  If not, see <http://www.gnu.org/licenses/>.
use std::io::{self, Write};
use std::mem;
use std::ops::{Deref, DerefMut};

mod cell;
mod char_grid;
mod cursor;
mod grid;
mod input;
mod styles;
mod tooltip;

use datatypes::{InputMode, Key};

pub use self::cell::CharCell;
pub use self::char_grid::CharGrid;
pub use self::cursor::Cursor;
pub use self::grid::Grid;
pub use self::styles::Styles;
pub use self::tooltip::Tooltip;

use self::input::Input;

pub struct Terminal {
    pub width: u32,
    pub height: u32,
    title: String,
    active: CharGrid,
    inactive: Vec<CharGrid>,
    tty: Input,
}

impl Terminal {

    pub fn new<W: Write + 'static>(width: u32, height: u32, tty: W) -> Terminal {
        let grid = CharGrid::new(width, height, false, true);
        let tty = Input::new(tty);
        Terminal {
            width: width,
            height: height,
            title: String::new(),
            active: grid,
            inactive: Vec::new(),
            tty: tty,
        }
    }

    pub fn send_input(&mut self, key: Key, press: bool) -> io::Result<()> {
        match key {
            Key::DownArrow | Key::UpArrow | Key::Enter if press => {
                let cursor = self.cursor_position();
                match match self.tooltip_at_mut(cursor) {
                    Some(tooltip @ &mut Tooltip::Menu { .. })   => tooltip.interact(&key),
                    _                                           => Err(true)
                } {
                    Ok(n)       => self.tty.write(Key::MenuSelection(n), true),
                    Err(true)   => self.tty.write(key, press),
                    Err(false)  => Ok(())
                }
            }
            _           => self.tty.write(key, press),
        }
    }

    pub fn push_buffer(&mut self, scroll_x: bool, scroll_y: bool) {
        let mut grid = CharGrid::new(self.width, self.height, scroll_x, scroll_y);
        mem::swap(&mut grid, &mut self.active);
        self.inactive.push(grid);
    }

    pub fn pop_buffer(&mut self) {
        self.inactive.pop().map(|grid| self.active = grid);
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_input_mode(&mut self, mode: InputMode) {
        self.tty.set_mode(mode);
    }

    pub fn bell(&mut self) {
        println!("BELL");
    }

    pub fn set_visible_height(&mut self, rows: u32) {
        self.active.set_height(rows);
        self.height = rows;
    }

    pub fn set_visible_width(&mut self, cols: u32) {
        self.active.set_width(cols);
        self.width = cols;
    }

}

impl Deref for Terminal {
    type Target = CharGrid;
    fn deref(&self) -> &CharGrid {
        &self.active
    }
}

impl DerefMut for Terminal {
    fn deref_mut(&mut self) -> &mut CharGrid {
        &mut self.active
    }
}
