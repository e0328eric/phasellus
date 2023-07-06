#![allow(clippy::unit_arg)]

mod playboard;
mod players;

use cursive::{
    theme::{Palette, Theme},
    traits::Nameable,
    Cursive, CursiveExt,
};

fn main() {
    let palette = Palette::terminal_default();
    let mut siv = Cursive::default();
    siv.set_theme(Theme {
        shadow: false,
        palette,
        ..Default::default()
    });

    let board = playboard::PlayBoard::new();

    siv.add_layer(board.with_name("playboard"));

    siv.run();
}
