#[macro_export]
macro_rules! score_event {
    ($title: literal, $score_input: expr) => {
        EventResult::with_cb_once(|siv| {
            siv.add_layer(
                Dialog::new().title($title).content(
                    LinearLayout::vertical()
                        .child(TextView::new("Input the score"))
                        .child(
                            OnEventView::new(
                                EditView::new()
                                    .on_submit(|s, _| get_score_input(s, $title, $score_input))
                                    .with_name("get_score"),
                            )
                            .on_event(Key::Esc, |s| {
                                s.pop_layer();
                            }),
                        )
                        .child(
                            LinearLayout::horizontal()
                                .child(Button::new("Ok", |s| {
                                    get_score_input(s, $title, $score_input)
                                }))
                                .child(Button::new("Cancel", |s| {
                                    s.pop_layer();
                                })),
                        ),
                ),
            );
        })
    };
}
