#[macro_use]
mod macros;
mod infobox;

use std::cell::Cell;
use std::cmp;

use cursive::{
    event::{Event, EventResult, Key},
    traits::Nameable,
    view::View,
    views::{Button, Dialog, EditView, LinearLayout, OnEventView, TextView},
    Cursive, Printer, Vec2,
};

use crate::players::{Players, ScoreInput};

// Draw Scoreboard lines
const HORIZONTAL_LINE: &str = "─";
const VERTICAL_LINE: &str = "│";
const TOP_LEFT_CORNER: &str = "┌";
const TOP_RIGHT_CORNER: &str = "┐";
const BOTTOM_LEFT_CORNER: &str = "└";
const BOTTOM_RIGHT_CORNER: &str = "┘";
const HORIZ_DOWN: &str = "┬";
const HORIZ_UP: &str = "┴";
const VERT_LEFT: &str = "┤";
const VERT_RIGHT: &str = "├";
const HORIZ_VERT: &str = "┼";

const MIN_PLAYER_NAME_OFFSET: usize = 6;

pub struct PlayBoard {
    players: Players,
    x_offset: Cell<usize>,
    y_offset: Cell<usize>,
    username_offset: Cell<usize>,
    width: Cell<usize>,
}

impl PlayBoard {
    pub fn new() -> Self {
        Self {
            players: Players::new(),
            x_offset: Cell::new(1),
            y_offset: Cell::new(1),
            username_offset: Cell::new(25),
            width: Cell::new(0),
        }
    }

    fn draw_init(&self, printer: &Printer) {
        let (term_x, term_y) = printer.size.pair();

        if self.players.is_empty() {
            self.width.set(25);
        }
        self.x_offset
            .set(term_x.saturating_sub(self.width.get()).saturating_div(2));
        self.y_offset
            .set(term_y.saturating_div(2).saturating_sub(11));

        let x_offset = self.x_offset.get();
        let y_offset = self.y_offset.get();
        let username_offset = self.username_offset.get();

        printer.print(
            (1, term_y.saturating_sub(1)),
            "Press `?` to show the help message.",
        );

        for i in 1..username_offset {
            printer.print((x_offset + i, y_offset), HORIZONTAL_LINE);
            printer.print((x_offset + i, y_offset + 2), HORIZONTAL_LINE);
            printer.print((x_offset + i, y_offset + 9), HORIZONTAL_LINE);
            printer.print((x_offset + i, y_offset + 12), HORIZONTAL_LINE);
            printer.print((x_offset + i, y_offset + 19), HORIZONTAL_LINE);
            printer.print((x_offset + i, y_offset + 20), HORIZONTAL_LINE);
            printer.print((x_offset + i, y_offset + 22), HORIZONTAL_LINE);
        }
        for i in y_offset..(y_offset + 22) {
            printer.print((x_offset, i), VERTICAL_LINE);
            printer.print(
                (x_offset + username_offset.saturating_sub(1), i),
                VERTICAL_LINE,
            );
            printer.print((x_offset + username_offset, i), VERTICAL_LINE);
        }
        printer.print((x_offset, y_offset), TOP_LEFT_CORNER);
        printer.print((x_offset + username_offset, y_offset), TOP_RIGHT_CORNER);
        printer.print(
            (x_offset + username_offset, y_offset + 22),
            BOTTOM_RIGHT_CORNER,
        );
        printer.print((x_offset, y_offset + 22), BOTTOM_LEFT_CORNER);

        printer.print(
            (x_offset + username_offset.saturating_sub(1), y_offset),
            HORIZ_DOWN,
        );
        printer.print(
            (x_offset + username_offset.saturating_sub(1), y_offset + 2),
            HORIZ_VERT,
        );
        printer.print(
            (x_offset + username_offset.saturating_sub(1), y_offset + 9),
            HORIZ_VERT,
        );
        printer.print(
            (x_offset + username_offset.saturating_sub(1), y_offset + 12),
            HORIZ_VERT,
        );
        printer.print(
            (x_offset + username_offset.saturating_sub(1), y_offset + 19),
            HORIZ_VERT,
        );
        printer.print(
            (x_offset + username_offset.saturating_sub(1), y_offset + 20),
            HORIZ_VERT,
        );
        printer.print(
            (x_offset + username_offset.saturating_sub(1), y_offset + 22),
            HORIZ_UP,
        );

        printer.print((x_offset, y_offset + 2), VERT_RIGHT);
        printer.print((x_offset, y_offset + 9), VERT_RIGHT);
        printer.print((x_offset, y_offset + 12), VERT_RIGHT);
        printer.print((x_offset, y_offset + 19), VERT_RIGHT);
        printer.print((x_offset, y_offset + 20), VERT_RIGHT);

        printer.print((x_offset + username_offset, y_offset + 2), VERT_LEFT);
        printer.print((x_offset + username_offset, y_offset + 9), VERT_LEFT);
        printer.print((x_offset + username_offset, y_offset + 12), VERT_LEFT);
        printer.print((x_offset + username_offset, y_offset + 19), VERT_LEFT);
        printer.print((x_offset + username_offset, y_offset + 20), VERT_LEFT);

        //
        printer.print((x_offset + 10, y_offset + 1), "Name");

        printer.print((x_offset + 8, y_offset + 3), "Ones   (1)");
        printer.print((x_offset + 8, y_offset + 4), "Twos   (2)");
        printer.print((x_offset + 8, y_offset + 5), "Threes (3)");
        printer.print((x_offset + 8, y_offset + 6), "Fours  (4)");
        printer.print((x_offset + 8, y_offset + 7), "Fives  (5)");
        printer.print((x_offset + 8, y_offset + 8), "Sixes  (6)");

        printer.print((x_offset + 4, y_offset + 10), "Left to get bonus");
        printer.print((x_offset + 10, y_offset + 11), "Bonus");

        printer.print((x_offset + 8, y_offset + 13), "Choice     (c)");
        printer.print((x_offset + 6, y_offset + 14), "Full House   (h)");
        printer.print((x_offset + 4, y_offset + 15), "Four of a kind (k)");
        printer.print((x_offset + 4, y_offset + 16), "Small Straight (s)");
        printer.print((x_offset + 4, y_offset + 17), "Large Straight (l)");
        printer.print((x_offset + 6, y_offset + 18), "* YACHT *    (y)");

        printer.print((x_offset + 10, y_offset + 21), "Total");
    }
}

impl View for PlayBoard {
    fn draw(&self, printer: &Printer) {
        self.draw_init(printer);

        let y_offset = self.y_offset.get();
        let mut offset = self.x_offset.get() + self.username_offset.get();

        // TODO: Support UTF-8 name
        for (player, scoreboard) in self.players.iter() {
            let name_offset = cmp::max(player.len() + 3, MIN_PLAYER_NAME_OFFSET);

            // Draw lines
            for i in 1..name_offset {
                printer.print((offset + i, y_offset), HORIZONTAL_LINE);
                printer.print((offset + i, y_offset + 2), HORIZONTAL_LINE);
                printer.print((offset + i, y_offset + 9), HORIZONTAL_LINE);
                printer.print((offset + i, y_offset + 12), HORIZONTAL_LINE);
                printer.print((offset + i, y_offset + 19), HORIZONTAL_LINE);
                printer.print((offset + i, y_offset + 20), HORIZONTAL_LINE);
                printer.print((offset + i, y_offset + 22), HORIZONTAL_LINE);
            }
            for i in y_offset..(y_offset + 22) {
                printer.print((offset, i), VERTICAL_LINE);
                printer.print((offset + name_offset, i), VERTICAL_LINE);
            }
            printer.print((offset, y_offset), HORIZ_DOWN);
            printer.print((offset + name_offset, y_offset), TOP_RIGHT_CORNER);
            printer.print((offset, y_offset + 22), HORIZ_UP);
            printer.print((offset + name_offset, y_offset + 22), BOTTOM_RIGHT_CORNER);

            printer.print((offset, y_offset + 2), HORIZ_VERT);
            printer.print((offset, y_offset + 9), HORIZ_VERT);
            printer.print((offset, y_offset + 12), HORIZ_VERT);
            printer.print((offset, y_offset + 19), HORIZ_VERT);
            printer.print((offset, y_offset + 20), HORIZ_VERT);

            printer.print((offset + name_offset, y_offset + 2), VERT_LEFT);
            printer.print((offset + name_offset, y_offset + 9), VERT_LEFT);
            printer.print((offset + name_offset, y_offset + 12), VERT_LEFT);
            printer.print((offset + name_offset, y_offset + 19), VERT_LEFT);
            printer.print((offset + name_offset, y_offset + 20), VERT_LEFT);

            // Fill Contents
            printer.print((offset + 2, self.y_offset.get() + 1), player);

            printer.print(
                (offset + 2, y_offset + 3),
                &format!("{}", scoreboard.numbers[0]),
            );
            printer.print(
                (offset + 2, y_offset + 4),
                &format!("{}", scoreboard.numbers[1]),
            );
            printer.print(
                (offset + 2, y_offset + 5),
                &format!("{}", scoreboard.numbers[2]),
            );
            printer.print(
                (offset + 2, y_offset + 6),
                &format!("{}", scoreboard.numbers[3]),
            );
            printer.print(
                (offset + 2, y_offset + 7),
                &format!("{}", scoreboard.numbers[4]),
            );
            printer.print(
                (offset + 2, y_offset + 8),
                &format!("{}", scoreboard.numbers[5]),
            );

            printer.print(
                (offset + 2, y_offset + 10),
                &format!("{}", scoreboard.left_to_get_bonus),
            );
            printer.print(
                (offset + 2, y_offset + 11),
                &format!("{}", scoreboard.bonus),
            );

            printer.print(
                (offset + 2, y_offset + 13),
                &format!("{}", scoreboard.choice),
            );
            printer.print(
                (offset + 2, y_offset + 14),
                &format!("{}", scoreboard.full_house),
            );
            printer.print(
                (offset + 2, y_offset + 15),
                &format!("{}", scoreboard.four_of_kind),
            );
            printer.print(
                (offset + 2, y_offset + 16),
                &format!("{}", scoreboard.small_straight),
            );
            printer.print(
                (offset + 2, y_offset + 17),
                &format!("{}", scoreboard.large_straight),
            );
            printer.print(
                (offset + 2, y_offset + 18),
                &format!("{}", scoreboard.yacht),
            );

            printer.print(
                (offset + 2, y_offset + 21),
                &format!("{}", scoreboard.total_score),
            );

            offset += cmp::max(player.len() + 3, MIN_PLAYER_NAME_OFFSET);
        }
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        constraint
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Char('q') => EventResult::with_cb(|siv| siv.quit()),
            Event::Char('?') => EventResult::with_cb(|siv| siv.add_layer(infobox::InfoBox)),
            Event::Char('C') => EventResult::with_cb_once(|siv| {
                siv.call_on_name("playboard", |play_board: &mut PlayBoard| {
                    play_board.players.clear_score();
                });
            }),
            Event::Char('a') => EventResult::with_cb_once(move |siv| {
                siv.add_layer(
                    Dialog::new().title("Add Player").content(
                        LinearLayout::vertical()
                            .child(TextView::new("Input a player name to add"))
                            .child(
                                OnEventView::new(
                                    EditView::new()
                                        .on_submit(|s, _| update_player(s))
                                        .with_name("add_player_name"),
                                )
                                .on_event(Key::Esc, |s| {
                                    s.pop_layer();
                                }),
                            )
                            .child(
                                LinearLayout::horizontal()
                                    .child(Button::new("Ok", update_player))
                                    .child(Button::new("Cancel", |s| {
                                        s.pop_layer();
                                    })),
                            ),
                    ),
                );
            }),
            Event::Char('d') => EventResult::with_cb_once(move |siv| {
                siv.add_layer(
                    Dialog::new().title("Delete Player").content(
                        LinearLayout::vertical()
                            .child(TextView::new("Input a player name to remove"))
                            .child(
                                OnEventView::new(
                                    EditView::new()
                                        .on_submit(|s, _| delete_player(s))
                                        .with_name("del_player_name"),
                                )
                                .on_event(Key::Esc, |s| {
                                    s.pop_layer();
                                }),
                            )
                            .child(
                                LinearLayout::horizontal()
                                    .child(Button::new("Ok", delete_player))
                                    .child(Button::new("Cancel", |s| {
                                        s.pop_layer();
                                    })),
                            ),
                    ),
                );
            }),
            Event::Char('1') => score_event!("Ones", ScoreInput::Ones(None)),
            Event::Char('2') => score_event!("Twos", ScoreInput::Twos(None)),
            Event::Char('3') => score_event!("Threes", ScoreInput::Threes(None)),
            Event::Char('4') => score_event!("Fours", ScoreInput::Fours(None)),
            Event::Char('5') => score_event!("Fives", ScoreInput::Fives(None)),
            Event::Char('6') => score_event!("Sixes", ScoreInput::Sixes(None)),
            Event::Char('c') => score_event!("Choice", ScoreInput::Choice(None)),
            Event::Char('h') => score_event!("Full House", ScoreInput::FullHouse(None)),
            Event::Char('k') => score_event!("Four of a Kind", ScoreInput::FourOfKind(None)),
            Event::Char('s') => EventResult::with_cb_once(move |siv| {
                siv.add_layer(
                    Dialog::new().title("Small Straight").content(
                        LinearLayout::vertical()
                            .child(TextView::new("Input the player name"))
                            .child(
                                OnEventView::new(
                                    EditView::new()
                                        .on_submit(move |s, _| {
                                            update_player_score(s, ScoreInput::SmallStraight)
                                        })
                                        .with_name("update_player_score"),
                                )
                                .on_event(Key::Esc, |s| {
                                    s.pop_layer();
                                }),
                            )
                            .child(
                                LinearLayout::horizontal()
                                    .child(Button::new("Ok", move |s| {
                                        update_player_score(s, ScoreInput::SmallStraight)
                                    }))
                                    .child(Button::new("Cancel", |s| {
                                        s.pop_layer();
                                    })),
                            ),
                    ),
                );
            }),
            Event::Char('l') => EventResult::with_cb_once(move |siv| {
                siv.add_layer(
                    Dialog::new().title("Large Straight").content(
                        LinearLayout::vertical()
                            .child(TextView::new("Input the player name"))
                            .child(
                                OnEventView::new(
                                    EditView::new()
                                        .on_submit(move |s, _| {
                                            update_player_score(s, ScoreInput::LargeStraight)
                                        })
                                        .with_name("update_player_score"),
                                )
                                .on_event(Key::Esc, |s| {
                                    s.pop_layer();
                                }),
                            )
                            .child(
                                LinearLayout::horizontal()
                                    .child(Button::new("Ok", move |s| {
                                        update_player_score(s, ScoreInput::LargeStraight)
                                    }))
                                    .child(Button::new("Cancel", |s| {
                                        s.pop_layer();
                                    })),
                            ),
                    ),
                );
            }),
            Event::Char('y') => EventResult::with_cb_once(move |siv| {
                siv.add_layer(
                    Dialog::new().title("* YACHT *").content(
                        LinearLayout::vertical()
                            .child(TextView::new("Input the player name"))
                            .child(
                                OnEventView::new(
                                    EditView::new()
                                        .on_submit(move |s, _| {
                                            update_player_score(s, ScoreInput::Yacht)
                                        })
                                        .with_name("update_player_score"),
                                )
                                .on_event(Key::Esc, |s| {
                                    s.pop_layer();
                                }),
                            )
                            .child(
                                LinearLayout::horizontal()
                                    .child(Button::new("Ok", move |s| {
                                        update_player_score(s, ScoreInput::Yacht)
                                    }))
                                    .child(Button::new("Cancel", |s| {
                                        s.pop_layer();
                                    })),
                            ),
                    ),
                );
            }),
            _ => EventResult::Ignored,
        }
    }
}

fn update_player(siv: &mut Cursive) {
    let player_name = siv.call_on_name("add_player_name", |view: &mut EditView| view.get_content());

    if let Some(name) = player_name {
        siv.call_on_name("playboard", |play_board: &mut PlayBoard| {
            play_board.players.add_player(&name);
            let width = play_board.width.get();
            play_board
                .width
                .set(width + cmp::max(name.len() + 3, MIN_PLAYER_NAME_OFFSET));
        });
    }

    siv.pop_layer();
}

fn delete_player(siv: &mut Cursive) {
    let player_name = siv.call_on_name("del_player_name", |view: &mut EditView| view.get_content());

    if let Some(name) = player_name {
        let well_removed = siv
            .call_on_name("playboard", |play_board: &mut PlayBoard| {
                let width = play_board.width.get();
                play_board
                    .width
                    .set(width.saturating_sub(cmp::max(name.len() + 3, MIN_PLAYER_NAME_OFFSET)));
                play_board.players.del_player(&name)
            })
            .expect("`playboard` must exists");

        if !well_removed {
            siv.add_layer(
                Dialog::new().title("Cannot Delete Player").content(
                    LinearLayout::vertical()
                        .child(TextView::new("There is no player to remove from the list"))
                        .child(Button::new("Ok", |s| {
                            s.pop_layer();
                        })),
                ),
            );
        }
    }

    siv.pop_layer();
}

fn get_score_input(siv: &mut Cursive, title: &str, score_input: ScoreInput) {
    let score_str = siv.call_on_name("get_score", |view: &mut EditView| view.get_content());

    if let Some(score) = score_str {
        let Ok(score) = score.trim().parse::<u16>() else {
            siv.pop_layer();
            return;
        };
        siv.pop_layer();
        siv.add_layer(
            Dialog::new().title(title).content(
                LinearLayout::vertical()
                    .child(TextView::new("Input the player name"))
                    .child(
                        OnEventView::new(
                            EditView::new()
                                .on_submit(move |s, _| {
                                    update_player_score(s, score_input.inject(score))
                                })
                                .with_name("update_player_score"),
                        )
                        .on_event(Key::Esc, |s| {
                            s.pop_layer();
                        }),
                    )
                    .child(
                        LinearLayout::horizontal()
                            .child(Button::new("Ok", move |s| {
                                update_player_score(s, score_input.inject(score))
                            }))
                            .child(Button::new("Cancel", |s| {
                                s.pop_layer();
                            })),
                    ),
            ),
        );
    }
}

fn update_player_score(siv: &mut Cursive, score: ScoreInput) {
    let player_name = siv.call_on_name("update_player_score", |view: &mut EditView| {
        view.get_content()
    });

    if let Some(name) = player_name {
        siv.call_on_name("playboard", |play_board: &mut PlayBoard| {
            play_board.players.calculate_score(&name, score);
        });
    }

    siv.pop_layer();
}