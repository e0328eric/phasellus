use cursive::{
    event::{Event, EventResult},
    view::View,
    views::Dialog,
    Printer, Vec2,
};

const INFOBOX_INNER_SIZE: (usize, usize) = (82, 18);

pub(super) struct InfoBox;

impl View for InfoBox {
    fn draw(&self, printer: &Printer) {
        let printer = printer.inner_size(INFOBOX_INNER_SIZE);
        Dialog::new().title("Help Message").draw(&printer);

        printer.print((2, 2), "Keybindings for phasellus program");

        printer.print((2, 4), "<Player Related Keybindings>");
        printer.print((2, 5), "a: add player");
        printer.print((2, 6), "d: delete player");
        printer.print((2, 7), "q: quit this program");

        printer.print((2, 9), "<Save and Load datas>");
        printer.print((2, 10), "Ctrl-s: save the data into JSON files");
        printer.print((2, 11), "Ctrl-l: load the data from JSON files");

        printer.print((43, 4), "<Score Related Keybindings>");
        printer.print((43, 5), "1 ~ 6: add score at ones, ..., sixes");
        printer.print((43, 6), "c: add score at choice");
        printer.print((43, 6), "h: add score at full house");
        printer.print((43, 7), "k: add score at four of a kind");
        printer.print((43, 8), "s: add score at small straight");
        printer.print((43, 9), "l: add score at large straight");
        printer.print((43, 10), "y: add score at yacht");
        printer.print((43, 11), "s: add score at small straight");
        printer.print((43, 12), "l: add score at large straight");
        printer.print((43, 13), "y: add score at yacht");
        printer.print((43, 14), "C: clear all scores");

        printer.print((2, 16), "Press `q` to close this help message");
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        INFOBOX_INNER_SIZE.into()
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Char('q') => EventResult::with_cb(|siv| {
                siv.pop_layer();
            }),
            _ => EventResult::Ignored,
        }
    }
}
