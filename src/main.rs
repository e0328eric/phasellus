use std::collections::HashMap;
use std::fmt::{self, Display, Write};
use std::io;

#[allow(unused_imports)]
use rustyline::{error::ReadlineError, Editor};
use unicode_width::UnicodeWidthStr;

const PROMPT: &str = ">> ";
const MOST_LEFT_PADDING: usize = 18;
const BONUS_LIMIT: u16 = 63;
const BONUS_SCORE: u16 = 35;
const LITTLE_STRAIGHT_SCORE: u16 = 15;
const BIG_STRAIGHT_SCORE: u16 = 30;
const YACHT_SCORE: u16 = 50;

type Players = HashMap<String, Scoreboard>;
type Result<T> = std::result::Result<T, YachtErr>;

#[derive(Debug, Default, Clone, Copy)]
struct Scoreboard {
    numbers: [Option<u16>; 6],
    sum: u16,
    bonus: bool,
    choice: Option<u16>,
    full_house: Option<u16>,
    four_of_kind: Option<u16>,
    little_straight: Option<bool>,
    big_straight: Option<bool>,
    yacht: Option<bool>,
    total_score: u16,
}

#[derive(Debug)]
enum YachtErr {
    RustylineErr(ReadlineError),
    NoNameWasGiven,
    NoArgumentWasGiven,
    InvalidPlayerName,
    InvalidScoringName,
}

impl From<std::io::Error> for YachtErr {
    fn from(err: io::Error) -> Self {
        Self::RustylineErr(ReadlineError::Io(err))
    }
}

impl From<ReadlineError> for YachtErr {
    fn from(err: ReadlineError) -> Self {
        Self::RustylineErr(err)
    }
}

fn main() -> Result<()> {
    let mut rl = Editor::<()>::new()?;

    let mut players = Players::new();

    loop {
        let readline = rl.readline(PROMPT);

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let (command, rest) = if let Some((command, rest)) = line.trim().split_once(' ') {
                    (command, Some(rest))
                } else {
                    (line.trim(), None)
                };
                let result = match command {
                    "quit" | "q" => break,
                    "addplayer" | "aplayer" | "ap" => add_player(&mut players, rest),
                    "delplayer" | "dplayer" | "dp" => del_player(&mut players, rest),
                    "show" => Ok(show_scores(&players)),
                    "score" => calculate_score(&mut players, rest),
                    "reset" => reset_score(&mut players, rest),
                    "clear" => Ok(clear_score(&mut players)),
                    "help" | "h" => Ok(display_help()),
                    "import" => todo!("Not support yet"),
                    "export" => unimplemented!(),
                    _ => Ok(()),
                };

                match result {
                    Ok(()) => {}
                    Err(YachtErr::RustylineErr(err)) => {
                        println!("ERROR: {err:?}");
                        break;
                    }
                    Err(err) => println!("ERROR: {err:?}\n"),
                }
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => break,
            Err(err) => {
                println!("ERROR: {err:?}");
                break;
            }
        }
    }

    Ok(())
}

fn add_player(players: &mut Players, rest: Option<&str>) -> Result<()> {
    let mut name = String::new();
    let iter = rest.ok_or(YachtErr::NoNameWasGiven)?.split_whitespace();

    for s in iter {
        name.push_str(s);
        name.push(' ');
    }

    if name.is_empty() {
        Err(YachtErr::NoNameWasGiven)
    } else {
        name.pop();
        players.insert(name, Scoreboard::default());

        Ok(())
    }
}

fn del_player(players: &mut Players, rest: Option<&str>) -> Result<()> {
    let mut name = String::new();
    let iter = rest.ok_or(YachtErr::NoNameWasGiven)?.split_whitespace();

    for s in iter {
        name.push_str(s);
    }

    if name.is_empty() {
        Err(YachtErr::NoNameWasGiven)
    } else {
        players.remove(&name);

        Ok(())
    }
}

// TODO: make well parser
fn parse_rest_str(rest: &str) -> Result<(&str, &str, Option<u16>)> {
    let mut iter = rest.split_whitespace();

    let name = iter.next().ok_or(YachtErr::InvalidPlayerName)?;
    let scoring = iter.next().ok_or(YachtErr::InvalidScoringName)?;
    let score_num = match iter.next() {
        Some("true" | "t") => Some(1),
        Some("false" | "f") => Some(0),
        Some(score) => score.parse::<u16>().ok(),
        None => None,
    };

    Ok((name, scoring, score_num))
}

fn calculate_score(players: &mut Players, rest: Option<&str>) -> Result<()> {
    let (name, scoring, score_num) = parse_rest_str(rest.ok_or(YachtErr::NoArgumentWasGiven)?)?;

    let score_board = players.get_mut(name).ok_or(YachtErr::InvalidPlayerName)?;

    match scoring {
        "1s" | "ones" => score_board.numbers[0] = score_num,
        "2s" | "twos" => score_board.numbers[1] = score_num,
        "3s" | "threes" => score_board.numbers[2] = score_num,
        "4s" | "fours" => score_board.numbers[3] = score_num,
        "5s" | "fives" => score_board.numbers[4] = score_num,
        "6s" | "sixes" => score_board.numbers[5] = score_num,
        "choice" | "c" => score_board.choice = score_num,
        "fullhouse" | "fh" => score_board.full_house = score_num,
        "fourcards" | "fourcard" | "fc" | "fk" => score_board.four_of_kind = score_num,
        "ss" => score_board.little_straight = Some(score_num != Some(0)),
        "ls" => score_board.big_straight = Some(score_num != Some(0)),
        "yacht" | "y" => score_board.yacht = Some(score_num != Some(0)),
        "fuck" | "fucked" => score_board.yacht = Some(false),
        _ => return Err(YachtErr::InvalidScoringName),
    }

    score_board.sum = score_board
        .numbers
        .iter()
        .filter_map(|nums| nums.as_ref())
        .sum::<u16>();
    if score_board.sum >= BONUS_LIMIT {
        score_board.bonus = true;
    }

    score_board.total_score = {
        let bonus_score = if score_board.bonus { BONUS_SCORE } else { 0 };
        let choice_num = score_board.choice.unwrap_or_default();
        let full_house_num = score_board.full_house.unwrap_or_default();
        let four_of_kind_num = score_board.four_of_kind.unwrap_or_default();
        let little_straight_score = if score_board.little_straight == Some(true) {
            LITTLE_STRAIGHT_SCORE
        } else {
            0
        };
        let big_straight_score = if score_board.big_straight == Some(true) {
            BIG_STRAIGHT_SCORE
        } else {
            0
        };
        let yacht_score = if score_board.yacht == Some(true) {
            YACHT_SCORE
        } else {
            0
        };

        score_board.sum
            + bonus_score
            + choice_num
            + full_house_num
            + four_of_kind_num
            + little_straight_score
            + big_straight_score
            + yacht_score
    };

    show_scores(players);

    Ok(())
}

fn clear_score(players: &mut Players) {
    for (_, player) in players.iter_mut() {
        *player = Scoreboard::default();
    }
}

fn reset_score(players: &mut Players, rest: Option<&str>) -> Result<()> {
    let (name, scoring, _) = parse_rest_str(rest.ok_or(YachtErr::NoArgumentWasGiven)?)?;
    let score_board = players.get_mut(name).ok_or(YachtErr::InvalidPlayerName)?;

    match scoring {
        "1s" | "ones" => score_board.numbers[0] = None,
        "2s" | "twos" => score_board.numbers[1] = None,
        "3s" | "threes" => score_board.numbers[2] = None,
        "4s" | "fours" => score_board.numbers[3] = None,
        "5s" | "fives" => score_board.numbers[4] = None,
        "6s" | "sixes" => score_board.numbers[5] = None,
        "choice" | "c" => score_board.choice = None,
        "full_house" | "fullhouse" | "fh" => score_board.full_house = None,
        "fourcards" | "fourcard" | "fc" | "fk" => score_board.four_of_kind = None,
        "ss" => score_board.little_straight = None,
        "ls" => score_board.big_straight = None,
        "yacht" | "y" => score_board.yacht = None,
        "all" => *score_board = Scoreboard::default(),
        _ => return Err(YachtErr::InvalidScoringName),
    }

    score_board.sum = score_board
        .numbers
        .iter()
        .filter_map(|nums| nums.as_ref())
        .sum::<u16>();
    if score_board.sum < BONUS_LIMIT {
        score_board.bonus = false;
    }

    score_board.total_score = {
        let bonus_score = if score_board.bonus { BONUS_SCORE } else { 0 };
        let choice_num = score_board.choice.unwrap_or_default();
        let full_house_num = score_board.full_house.unwrap_or_default();
        let four_of_kind_num = score_board.four_of_kind.unwrap_or_default();
        let little_straight_score = if score_board.little_straight == Some(true) {
            LITTLE_STRAIGHT_SCORE
        } else {
            0
        };
        let big_straight_score = if score_board.big_straight == Some(true) {
            BIG_STRAIGHT_SCORE
        } else {
            0
        };
        let yacht_score = if score_board.yacht == Some(true) {
            YACHT_SCORE
        } else {
            0
        };

        score_board.sum
            + bonus_score
            + choice_num
            + full_house_num
            + four_of_kind_num
            + little_straight_score
            + big_straight_score
            + yacht_score
    };

    show_scores(players);

    Ok(())
}

// implementing pretty printer
struct PrettyPrinter<'info> {
    info: Vec<(&'info str, &'info Scoreboard)>,
    padding: usize,
}

impl<'info> PrettyPrinter<'info> {
    #[inline]
    fn new(players: &'info Players) -> Self {
        let mut info = Vec::new();
        let mut padding = 4;

        for (player, score) in players {
            padding = std::cmp::max(padding, UnicodeWidthStr::width(player.as_str()));
            info.push((player.as_str(), score));
        }

        Self { info, padding }
    }
}

impl Display for PrettyPrinter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut topline = String::new();
        write!(
            topline,
            "| {:^width$} |",
            "score",
            width = MOST_LEFT_PADDING
        )?;
        for (player, _) in &self.info {
            write!(topline, "| {:^width$} ", player, width = self.padding)?;
        }
        writeln!(topline, "|")?;

        // Display the top line
        writeln!(
            f,
            "{:-<width$}",
            "",
            width = UnicodeWidthStr::width(topline.as_str())
        )?;
        write!(f, "{}", topline)?;
        writeln!(
            f,
            "{:-<width$}",
            "",
            width = UnicodeWidthStr::width(topline.as_str())
        )?;

        // Display numbers score
        for (i, scoring) in ["ones", "twos", "threes", "fours", "fives", "sixes"]
            .iter()
            .enumerate()
        {
            write!(f, "| {:^width$} |", scoring, width = MOST_LEFT_PADDING)?;
            for (_, score_board) in &self.info {
                write!(
                    f,
                    "| {:^width$} ",
                    if let Some(num) = score_board.numbers[i] {
                        num.to_string()
                    } else {
                        String::new()
                    },
                    width = self.padding
                )?;
            }
            writeln!(f, "|")?;
        }
        writeln!(
            f,
            "{:-<width$}",
            "",
            width = UnicodeWidthStr::width(topline.as_str())
        )?;

        // left bonus
        write!(
            f,
            "| {:^width$} |",
            "left to get bonus",
            width = MOST_LEFT_PADDING
        )?;
        for (_, score_board) in &self.info {
            write!(
                f,
                "| {:^width$} ",
                BONUS_LIMIT.saturating_sub(score_board.sum),
                width = self.padding
            )?;
        }
        writeln!(f, "|")?;

        // bonus
        write!(f, "| {:^width$} |", "bonus", width = MOST_LEFT_PADDING)?;
        for (_, score_board) in &self.info {
            write!(
                f,
                "| {:^width$} ",
                if score_board.bonus { BONUS_SCORE } else { 0 },
                width = self.padding
            )?;
        }
        writeln!(f, "|")?;

        writeln!(
            f,
            "{:-<width$}",
            "",
            width = UnicodeWidthStr::width(topline.as_str())
        )?;

        // Display scoring score
        // choice
        write!(f, "| {:^width$} |", "choice", width = MOST_LEFT_PADDING)?;
        for (_, score_board) in &self.info {
            write!(
                f,
                "| {:^width$} ",
                if let Some(num) = score_board.choice {
                    num.to_string()
                } else {
                    String::new()
                },
                width = self.padding
            )?;
        }
        writeln!(f, "|")?;

        // full house
        write!(f, "| {:^width$} |", "full house", width = MOST_LEFT_PADDING)?;
        for (_, score_board) in &self.info {
            write!(
                f,
                "| {:^width$} ",
                if let Some(num) = score_board.full_house {
                    num.to_string()
                } else {
                    String::new()
                },
                width = self.padding
            )?;
        }
        writeln!(f, "|")?;

        // four of a kind
        write!(
            f,
            "| {:^width$} |",
            "four of a kind",
            width = MOST_LEFT_PADDING
        )?;
        for (_, score_board) in &self.info {
            write!(
                f,
                "| {:^width$} ",
                if let Some(num) = score_board.four_of_kind {
                    num.to_string()
                } else {
                    String::new()
                },
                width = self.padding
            )?;
        }
        writeln!(f, "|")?;

        // little straight
        write!(
            f,
            "| {:^width$} |",
            "small straight",
            width = MOST_LEFT_PADDING
        )?;
        for (_, score_board) in &self.info {
            write!(
                f,
                "| {:^width$} ",
                match score_board.little_straight {
                    Some(true) => LITTLE_STRAIGHT_SCORE.to_string(),
                    Some(false) => 0.to_string(),
                    None => String::new(),
                },
                width = self.padding
            )?;
        }
        writeln!(f, "|")?;

        // big straight
        write!(
            f,
            "| {:^width$} |",
            "large straight",
            width = MOST_LEFT_PADDING
        )?;
        for (_, score_board) in &self.info {
            write!(
                f,
                "| {:^width$} ",
                match score_board.big_straight {
                    Some(true) => BIG_STRAIGHT_SCORE.to_string(),
                    Some(false) => 0.to_string(),
                    None => String::new(),
                },
                width = self.padding
            )?;
        }
        writeln!(f, "|")?;

        // yacht
        write!(f, "| {:^width$} |", "* yacht *", width = MOST_LEFT_PADDING)?;
        for (_, score_board) in &self.info {
            write!(
                f,
                "| {:^width$} ",
                match score_board.yacht {
                    Some(true) => YACHT_SCORE.to_string(),
                    Some(false) => 0.to_string(),
                    None => String::new(),
                },
                width = self.padding
            )?;
        }
        writeln!(f, "|")?;

        writeln!(
            f,
            "{:-<width$}",
            "",
            width = UnicodeWidthStr::width(topline.as_str())
        )?;
        writeln!(
            f,
            "{:-<width$}",
            "",
            width = UnicodeWidthStr::width(topline.as_str())
        )?;

        // total
        write!(f, "| {:^width$} |", "total", width = MOST_LEFT_PADDING)?;
        for (_, score_board) in &self.info {
            write!(
                f,
                "| {:^width$} ",
                score_board.total_score,
                width = self.padding
            )?;
        }
        writeln!(f, "|")?;
        writeln!(
            f,
            "{:-<width$}",
            "",
            width = UnicodeWidthStr::width(topline.as_str())
        )?;

        Ok(())
    }
}

fn show_scores(players: &Players) {
    println!("{}", PrettyPrinter::new(players));
}

fn display_help() {
    println!("addplayer, aplayer, ap [username]: add new player");
    println!("delplayer, dplayer, dp [username]: delete new player");
    println!("show: show scores\n");
    println!("score [username] [mod] [score number]: add score to the user");
    println!("<mod list>");
    println!("    | 1s, ones");
    println!("    | 2s, twos");
    println!("    | 3s, threes");
    println!("    | 4s, fours");
    println!("    | 5s, fives");
    println!("    | 6s, sixes");
    println!("    | choice, c");
    println!("    | fullhouse, fh");
    println!("    | fourcards, fourcard, fc, fk");
    println!("    | bs");
    println!("    | ls");
    println!("    | yacht, y\n");
    println!("reset [username] [mod]: reset score to the user");
    println!("<mod list>");
    println!("    | 1s, ones");
    println!("    | 2s, twos");
    println!("    | 3s, threes");
    println!("    | 4s, fours");
    println!("    | 5s, fives");
    println!("    | 6s, sixes");
    println!("    | choice, c");
    println!("    | fullhouse, fh");
    println!("    | fourcards, fourcard, fc, fk");
    println!("    | ss");
    println!("    | ls");
    println!("    | yacht, y");
    println!("    | all\n");
    println!("import: import informations from the excel file");
    println!("export: export the result into the excel file");
    println!("help, h: show this help message");
    println!("quit, q: quit the program");
}
