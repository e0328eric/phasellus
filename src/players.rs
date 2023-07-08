#![allow(unused)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

const BONUS_LIMIT: u16 = 63;
const BONUS_SCORE: u16 = 35;
const SMALL_STRAIGHT_SCORE: u16 = 15;
const LARGE_STRAIGHT_SCORE: u16 = 30;
const YACHT_SCORE: u16 = 50;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Scoreboard {
    pub numbers: [Option<u16>; 6],
    pub left_to_get_bonus: u16,
    pub bonus: u16,
    pub choice: Option<u16>,
    pub full_house: Option<u16>,
    pub four_of_kind: Option<u16>,
    pub small_straight: Option<u16>,
    pub large_straight: Option<u16>,
    pub yacht: Option<u16>,
    pub total_score: u16,
}

impl Default for Scoreboard {
    fn default() -> Self {
        Self {
            numbers: [None; 6],
            left_to_get_bonus: BONUS_LIMIT,
            bonus: 0,
            choice: None,
            full_house: None,
            four_of_kind: None,
            small_straight: None,
            large_straight: None,
            yacht: None,
            total_score: 0,
        }
    }
}

#[derive(Clone, Copy)]
pub enum ScoreInput {
    Ones(Option<u16>),
    Twos(Option<u16>),
    Threes(Option<u16>),
    Fours(Option<u16>),
    Fives(Option<u16>),
    Sixes(Option<u16>),
    Choice(Option<u16>),
    FullHouse(Option<u16>),
    FourOfKind(Option<u16>),
    SmallStraight(bool),
    LargeStraight(bool),
    Yacht(bool),
}

impl ScoreInput {
    pub fn inject(self, num: u16) -> Self {
        match self {
            Self::Ones(None) => Self::Ones(Some(num)),
            Self::Twos(None) => Self::Twos(Some(num)),
            Self::Threes(None) => Self::Threes(Some(num)),
            Self::Fours(None) => Self::Fours(Some(num)),
            Self::Fives(None) => Self::Fives(Some(num)),
            Self::Sixes(None) => Self::Sixes(Some(num)),
            Self::Choice(None) => Self::Choice(Some(num)),
            Self::FullHouse(None) => Self::FullHouse(Some(num)),
            Self::FourOfKind(None) => Self::FourOfKind(Some(num)),
            _ => self,
        }
    }
}

type PlayerName = String;

#[derive(Serialize, Deserialize)]
pub struct Players {
    players: HashMap<PlayerName, Scoreboard>,
}

impl Players {
    pub fn new() -> Self {
        Self {
            players: HashMap::with_capacity(10),
        }
    }

    #[inline]
    pub fn get_player_score(&self, name: &PlayerName) -> Option<&Scoreboard> {
        self.players.get(name)
    }

    #[inline]
    pub fn get_player_score_mut(&mut self, name: &PlayerName) -> Option<&mut Scoreboard> {
        self.players.get_mut(name)
    }

    #[inline]
    pub fn add_player(&mut self, name: &str) {
        self.players.insert(name.to_string(), Scoreboard::default());
    }

    #[inline]
    pub fn del_player(&mut self, name: &str) -> bool {
        self.players.remove(name).is_some()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.players.is_empty()
    }

    pub fn calculate_score(&mut self, player_name: &str, score: ScoreInput) -> Option<()> {
        let scoreboard = self.players.get_mut(player_name)?;

        match score {
            ScoreInput::Ones(score) => scoreboard.numbers[0] = score,
            ScoreInput::Twos(score) => scoreboard.numbers[1] = score,
            ScoreInput::Threes(score) => scoreboard.numbers[2] = score,
            ScoreInput::Fours(score) => scoreboard.numbers[3] = score,
            ScoreInput::Fives(score) => scoreboard.numbers[4] = score,
            ScoreInput::Sixes(score) => scoreboard.numbers[5] = score,
            ScoreInput::Choice(score) => scoreboard.choice = score,
            ScoreInput::FullHouse(score) => scoreboard.full_house = score,
            ScoreInput::FourOfKind(score) => scoreboard.four_of_kind = score,
            ScoreInput::SmallStraight(b) => {
                scoreboard.small_straight = Some(if b { SMALL_STRAIGHT_SCORE } else { 0 })
            }
            ScoreInput::LargeStraight(b) => {
                scoreboard.large_straight = Some(if b { LARGE_STRAIGHT_SCORE } else { 0 })
            }
            ScoreInput::Yacht(b) => scoreboard.yacht = Some(if b { YACHT_SCORE } else { 0 }),
        }

        let nums_total = scoreboard
            .numbers
            .iter()
            .map(|num| num.unwrap_or(0))
            .sum::<u16>();
        scoreboard.left_to_get_bonus = BONUS_LIMIT.saturating_sub(nums_total);
        scoreboard.bonus = if scoreboard.left_to_get_bonus == 0 {
            BONUS_SCORE
        } else {
            0
        };

        scoreboard.total_score = nums_total
            + scoreboard.bonus
            + scoreboard.choice.unwrap_or(0)
            + scoreboard.full_house.unwrap_or(0)
            + scoreboard.four_of_kind.unwrap_or(0)
            + scoreboard.small_straight.unwrap_or(0)
            + scoreboard.large_straight.unwrap_or(0)
            + scoreboard.yacht.unwrap_or(0);

        Some(())
    }

    pub fn clear_score(&mut self) {
        for (_, player) in self.players.iter_mut() {
            *player = Scoreboard::default();
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&'_ PlayerName, &'_ Scoreboard)> {
        self.players.iter()
    }
}
