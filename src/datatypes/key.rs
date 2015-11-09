use std::borrow::Cow;

use datatypes::InputMode;
use datatypes::InputMode::*;

use self::Key::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Key {
    Char(bool, char),
    Up(bool),
    Down(bool),
    Left(bool),
    Right(bool),
    ShiftLeft(bool),
    ShiftRight(bool),
    CtrlLeft(bool),
    CtrlRight(bool),
    AltLeft(bool),
    AltRight(bool),
    MetaLeft(bool),
    MetaRight(bool),
    PageUp(bool),
    PageDown(bool),
    Home(bool),
    End(bool),
    Insert(bool),
    Delete(bool),
    CapsLock(bool),
    NumLock(bool),
    ScrollLock(bool),
    Function(bool, u8),
    Cmd(Cow<'static, str>),
}

impl Key {

    pub fn as_code(&self, mode: InputMode, modifiers: Modifiers) -> Option<String> {
        match mode {
            Ansi        => self.compatible_code(modifiers, true),
            Application => self.compatible_code(modifiers, false),
            Extended    => self.extended_code(modifiers),
        }
    }

    fn compatible_code(&self, modifiers: Modifiers, ansi: bool) -> Option<String> {
        match *self {
            Char(true, c)                       => char_key(modifiers, c),
            Cmd(ref s)                          => Some(String::from(&**s)),
            Up(true)                            => term_key(modifiers, 'A', ansi),
            Down(true)                          => term_key(modifiers, 'B', ansi),
            Left(true)                          => term_key(modifiers, 'D', ansi),
            Right(true)                         => term_key(modifiers, 'C', ansi),
            ShiftLeft(_)
                | ShiftRight(_)
                | CtrlLeft(_)
                | CtrlRight(_)
                | AltLeft(_)
                | AltRight(_)
                | CapsLock(_)                   => unreachable!(),
            MetaLeft(true) | MetaRight(true)    => None,
            PageUp(true)                        => tilde_key(modifiers, '5'),
            PageDown(true)                      => tilde_key(modifiers, '6'),
            Home(true)                          => term_key(modifiers, 'H', true),
            End(true)                           => term_key(modifiers, 'F', true),
            Insert(true)                        => tilde_key(modifiers, '2'),
            Delete(true)                        => tilde_key(modifiers, '3'),
            NumLock(_)                          => unimplemented!(),
            ScrollLock(_)                       => unimplemented!(),
            Function(..)                        => unimplemented!(),
            _                                   => None,
        }
    }

    fn extended_code(&self, modifiers: Modifiers) -> Option<String> {
        unimplemented!()
    }

}

#[derive(Copy, Clone)]
pub struct Modifiers {
    pub shift: bool,
    pub caps: bool,
    pub ctrl: bool,
    pub alt: bool,
}

impl Modifiers {
    pub fn new() -> Modifiers {
        Modifiers {
            shift: false,
            caps: false,
            ctrl: false,
            alt: false
        }
    }

    pub fn triplet(&self) -> (bool, bool, bool) {
        (self.shift || self.caps, self.ctrl, self.alt)
    }

}

fn char_key(modifiers: Modifiers, c: char) -> Option<String> {
    match (modifiers.ctrl, modifiers.alt) {
        (false,  false) => Some(c.to_string()),
        (true,   false) => match c {
            c @ '\x40'...'\x7f' => Some((((c as u8) & 0b00011111) as char).to_string()),
            _                   => None,
        },
        (false,  true)  => Some(format!("\x1b{}", c)),
        (true,   true)  => match c {
            c @ '\x40'...'\x7f' => Some(format!("\x1b{}", ((c as u8) & 0b00011111 ) as char)),
            _                   => None,
        }
    }
}

fn term_key(modifiers: Modifiers, term: char, ansi: bool) -> Option<String> {
    match modifiers.triplet() {
        (false, false, false) if ansi   => Some(format!("\x1b[{}", term)),
        (false, false, false)           => Some(format!("\x1bO{}", term)),
        (true,  false, false)           => Some(format!("\x1b[1;2{}", term)),
        (false, false, true)            => Some(format!("\x1b[1;3{}", term)),
        (true,  false, true)            => Some(format!("\x1b[1;4{}", term)),
        (false, true,  false)           => Some(format!("\x1b[1;5{}", term)),
        (true,  true,  false)           => Some(format!("\x1b[1;6{}", term)),
        (false, true,  true)            => Some(format!("\x1b[1;7{}", term)),
        (true,  true,  true)            => Some(format!("\x1b[1;8{}", term)),
    }
}

fn tilde_key(modifiers: Modifiers, init: char) -> Option<String> {
    match modifiers.triplet() {
        (false, false, false)           => Some(format!("\x1b[{}~", init)),
        (true,  false, false)           => Some(format!("\x1b[{};2~", init)),
        (false, false, true)            => Some(format!("\x1b[{};3~", init)),
        (true,  false, true)            => Some(format!("\x1b[{};4~", init)),
        (false, true,  false)           => Some(format!("\x1b[{};5~", init)),
        (true,  true,  false)           => Some(format!("\x1b[{};6~", init)),
        (false, true,  true)            => Some(format!("\x1b[{};7~", init)),
        (true,  true,  true)            => Some(format!("\x1b[{};8~", init)),
    }
}