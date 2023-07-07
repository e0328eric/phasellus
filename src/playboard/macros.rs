#[macro_export]
macro_rules! make_popup {
    ($name: literal, $title: literal, $msg: literal, $callback: expr) => {
        EventResult::with_cb_once(move |siv| {
            siv.add_layer(
                Dialog::new().title($title).content(
                    LinearLayout::vertical()
                        .child(TextView::new($msg))
                        .child(
                            OnEventView::new(EditView::new().with_name($name))
                                .on_event(Key::Enter, $callback)
                                .on_event(Key::Esc, |s| {
                                    s.pop_layer();
                                }),
                        )
                        .child(
                            LinearLayout::horizontal()
                                .child(Button::new("Ok", $callback))
                                .child(Button::new("Cancel", |s| {
                                    s.pop_layer();
                                })),
                        ),
                ),
            );
        })
    };
}

#[macro_export]
macro_rules! score_event {
    ($title: literal, $score_input: expr) => {
        make_popup!("get_score", $title, "Input the score", |s| get_score_input(
            s,
            $title,
            $score_input
        ))
    };
}
