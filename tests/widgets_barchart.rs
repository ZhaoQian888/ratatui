use ratatui::{
    backend::TestBackend,
    buffer::Buffer,
    style::{Color, Style},
    widgets::{Bar, BarChart, BarGroup, Block, Borders},
    Terminal,
};

#[test]
fn widgets_barchart_not_full_below_max_value() {
    let test_case = |expected| {
        let backend = TestBackend::new(30, 10);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                let size = f.size();
                let barchart = BarChart::default()
                    .block(Block::default().borders(Borders::ALL))
                    .data(&[("empty", 0), ("half", 50), ("almost", 99), ("full", 100)])
                    .max(100)
                    .bar_width(7)
                    .bar_gap(0);
                f.render_widget(barchart, size);
            })
            .unwrap();
        terminal.backend().assert_buffer(&expected);
    };

    // check that bars fill up correctly up to max value
    test_case(Buffer::with_lines(vec![
        "┌────────────────────────────┐",
        "│              ▇▇▇▇▇▇▇███████│",
        "│              ██████████████│",
        "│              ██████████████│",
        "│       ▄▄▄▄▄▄▄██████████████│",
        "│       █████████████████████│",
        "│       █████████████████████│",
        "│       ██50█████99█████100██│",
        "│ empty  half  almost  full  │",
        "└────────────────────────────┘",
    ]));
}

#[test]
fn widgets_barchart_group() {
    const TERMINAL_HEIGHT: u16 = 11u16;
    let test_case = |expected| {
        let backend = TestBackend::new(35, TERMINAL_HEIGHT);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                let size = f.size();
                let barchart = BarChart::default()
                    .block(Block::default().borders(Borders::ALL))
                    .data(
                        BarGroup::default().label("Mar".into()).bars(&[
                            Bar::default()
                                .value(10)
                                .label("C1".into())
                                .style(Style::default().fg(Color::Red))
                                .value_style(Style::default().fg(Color::Blue)),
                            Bar::default()
                                .value(20)
                                .style(Style::default().fg(Color::Green))
                                .text_value("20M".to_string()),
                        ]),
                    )
                    .data(&vec![("C1", 50u64), ("C2", 40u64)])
                    .data(&[("C1", 60u64), ("C2", 90u64)])
                    .data(&[("xx", 10u64), ("xx", 10u64)])
                    .group_gap(2)
                    .bar_width(4)
                    .bar_gap(1);
                f.render_widget(barchart, size);
            })
            .unwrap();
        terminal.backend().assert_buffer(&expected);
    };

    let mut expected = Buffer::with_lines(vec![
        "┌─────────────────────────────────┐",
        "│                             ████│",
        "│                             ████│",
        "│                        ▅▅▅▅ ████│",
        "│            ▇▇▇▇        ████ ████│",
        "│            ████ ████   ████ ████│",
        "│     ▄▄▄▄   ████ ████   ████ ████│",
        "│▆10▆ 20M█   █50█ █40█   █60█ █90█│",
        "│ C1          C1   C2     C1   C2 │",
        "│   Mar                           │",
        "└─────────────────────────────────┘",
    ]);

    for y in 1..(TERMINAL_HEIGHT - 3) {
        for x in 1..5 {
            expected.get_mut(x, y).set_fg(Color::Red);
            expected.get_mut(x + 5, y).set_fg(Color::Green);
        }
    }

    expected.get_mut(2, 7).set_fg(Color::Blue);
    expected.get_mut(3, 7).set_fg(Color::Blue);

    test_case(expected);
}
