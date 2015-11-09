//! The datatypes module defines the abstract datatypes used by other components of natty.
//! 
//! The types in this module are intended to be passed between modules. As a design restriction,
//! any methods on any type in this submodule are required to take the receiver immutably.
mod key;
mod movement;
mod region;
mod vector;

pub use self::key::{Key, Modifiers};
pub use self::movement::Movement;
pub use self::region::Region;
pub use self::vector::Vector;

pub mod args {
    pub use super::{Area, Coords, Color, InputMode, Movement, Region, Style};
    pub use super::Area::*;
    pub use super::InputMode::*;
    pub use super::Movement::*;
    pub use super::Style::*;
}

/// An abstractly defined section of the grid.
///
/// Areas can be defined in terms of the current cursor position and the bounds of the grid. They
/// are converted into concrete sections of the screen when commands using Areas are applied.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Area {
    /// The cell the cursor is in.
    CursorCell,
    /// The row the cursor is in.
    CursorRow,
    /// The column the cursor is in.
    CursorColumn,
    /// All cells the cursor would traverse through in performing a movement (including the cell
    /// the cursor is in now, and the cell it would end in).
    CursorTo(Movement),
    /// The rectangle bound in one corner by the cursor position and another by this coordinate.
    CursorBound(Coords),
    /// The entire screen.
    WholeScreen,
    /// A concrete rectangular section of the screen.
    Bound(Region),
    /// The rows between the two parameters, inclusive of the first but not the second.
    Rows(u32, u32),
    /// The columns between the two parameters, inclusive of the first but not the second.
    Columns(u32, u32),
    /// Everything below the row the cursor is in, the boolean determines if this is inclusive of
    /// the cursor or not (inclusive = true).
    BelowCursor(bool),
}

/// Data that could be placed in a character cell.
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum CellData {
    /// A single unicode code point.
    Char(char), 
    /// An extension code point such as U+301. Normally, writing this to the screen does not
    /// overwrite a cell, but instead applies it to the char in the cell.
    ExtensionChar(char),
    /// A multi code-point grapheme, such as a Hangul triplet.
    Grapheme(String),
    /// Non-character data, with a mime type and some binary data.
    Data(String, Vec<u8>),
}

/// A kind of escape code format (used for structuring response strings).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Code {
    ANSI,
    Natty,
}

/// A 24-bit rgb color sequence.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Color(pub u8, pub u8, pub u8);

/// A corodinate pair.
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
pub struct Coords {
    pub x: u32,
    pub y: u32,
}

/// A direction of movement across the grid.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// An event that can be sent to the input processor.
#[derive(Clone, Eq, PartialEq)]
pub enum InputEvent {
    /// Data which will be transmitted to the controlling process (usually keyboard input).
    Key(Key),
    /// A mode shift for how the processor should transmit data.
    Mode(InputMode),
}

/// The mode the input processor is in.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InputMode {
    /// ANSI-compatible mode.
    Ansi,
    /// ANSI-compatible mode with application arrow key input.
    Application,
    Extended,
}

/// Set rich text styles. Booleans represent on or off.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Style {
    /// Field is number of underlines (between 0 and 2).
    Underline(u8),
    Bold(bool),
    Italic(bool),
    Blink(bool),
    InvertColors(bool),
    Strikethrough(bool),
    Opacity(u8),
    FgColor(Color),
    FgColorCfg(Option<u8>),
    BgColor(Color),
    BgColorCfg(Option<u8>),
}
