use yewi_test::render;

use super::ansi::{Colour, Style, parse};
use super::*;
use crate::attrs;

/// The text a reader sees, with tags removed.
fn text_of(html: &str) -> String {
    let mut text = String::new();
    let mut in_tag = false;
    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            c if !in_tag => text.push(c),
            _ => {}
        }
    }
    text
}

fn parsed_text(input: &str) -> String {
    parse(input).into_iter().map(|span| span.text).collect()
}

#[test]
fn renders_a_named_group_with_its_session() {
    let html = render(|| {
        html! {
            <Terminal title="zsh">
                <TerminalCommand>{ "cargo build" }</TerminalCommand>
                <TerminalOutput>{ "Finished in 2.1s" }</TerminalOutput>
            </Terminal>
        }
    });
    assert!(html.contains(r#"role="group""#));
    assert!(html.contains(r#"aria-label="zsh""#));
    let text = text_of(&html);
    assert!(text.contains("zsh"));
    assert!(text.contains("cargo build"));
    assert!(text.contains("Finished in 2.1s"));
}

#[test]
fn has_an_accessible_name_without_a_title() {
    let html = render(|| html! { <Terminal /> });
    assert!(html.contains(r#"aria-label="Terminal""#));
    let html = render(|| html! { <Terminal title="zsh" label="Deploy log" /> });
    assert!(html.contains(r#"aria-label="Deploy log""#));
}

#[test]
fn every_controls_variant_is_distinguishable() {
    let rendered: Vec<String> = TerminalControls::ALL
        .into_iter()
        .map(|controls| render(move || html! { <Terminal title="zsh" {controls} /> }))
        .collect();
    for (i, a) in rendered.iter().enumerate() {
        for b in &rendered[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn only_a_scrolling_body_is_a_tab_stop() {
    let fixed = render(|| html! { <Terminal /> });
    assert!(!fixed.contains("tabindex"));
    for html in
        [render(|| html! { <Terminal max_rows=8 /> }), render(|| html! { <Terminal nowrap=true /> })]
    {
        assert!(html.contains(r#"tabindex="0""#));
    }
}

#[test]
fn following_output_is_a_live_log() {
    let html = render(|| html! { <Terminal title="deploy" follow=true max_rows=8 /> });
    assert!(html.contains(r#"role="log""#));
    assert!(html.contains(r#"aria-label="deploy output""#));
    assert!(!render(|| html! { <Terminal /> }).contains(r#"role="log""#));
}

#[test]
fn the_prompt_is_not_part_of_the_command_text() {
    let html = render(|| html! { <TerminalCommand prompt="PS>">{ "Get-Item" }</TerminalCommand> });
    assert!(html.contains(r#"data-prompt="PS&gt;""#) || html.contains(r#"data-prompt="PS>""#));
    assert_eq!(text_of(&html), "Get-Item");
}

#[test]
fn the_cursor_is_hidden_from_assistive_technology() {
    let html = render(|| html! { <TerminalCommand cursor=true /> });
    assert!(html.contains(r#"aria-hidden="true""#));
    assert!(!render(|| html! { <TerminalCommand /> }).contains("aria-hidden"));
}

#[test]
fn output_renders_ansi_text_without_escape_codes() {
    let html = render(|| html! { <TerminalOutput text={"\x1b[1;32m   Compiling\x1b[0m yewi v0.1.0\n"} /> });
    assert_eq!(text_of(&html), "   Compiling yewi v0.1.0\n");
    assert!(html.contains("<span"));
}

#[test]
fn output_tones_are_distinguishable() {
    let rendered: Vec<String> = Tone::ALL
        .into_iter()
        .map(|tone| render(move || html! { <TerminalOutput {tone}>{ "line" }</TerminalOutput> }))
        .collect();
    for (i, a) in rendered.iter().enumerate() {
        for b in &rendered[i + 1..] {
            assert_ne!(a, b);
        }
    }
}

#[test]
fn id_class_and_attrs_reach_each_root_element() {
    for html in [
        render(|| html! { <Terminal id="t" class="mine" attrs={attrs! { "data-x" => "1" }} /> }),
        render(|| html! { <TerminalCommand id="t" class="mine" attrs={attrs! { "data-x" => "1" }} /> }),
        render(|| html! { <TerminalOutput id="t" class="mine" attrs={attrs! { "data-x" => "1" }} /> }),
    ] {
        let root = &html[..html.find('>').unwrap()];
        assert!(root.contains(r#"id="t""#));
        assert!(root.contains("mine"));
        assert!(root.contains(r#"data-x="1""#));
    }
}

// ---- ANSI parsing ------------------------------------------------------------

#[test]
fn plain_text_is_one_unstyled_span() {
    let spans = parse("hello\nworld");
    assert_eq!(spans.len(), 1);
    assert!(spans[0].style.is_plain());
    assert_eq!(spans[0].text, "hello\nworld");
}

#[test]
fn sgr_codes_set_the_style() {
    let cases: [(&str, Style); 9] = [
        ("31", Style { fg: Some(Colour::Palette(1)), ..Style::default() }),
        ("92", Style { fg: Some(Colour::Palette(10)), ..Style::default() }),
        ("44", Style { bg: Some(Colour::Palette(4)), ..Style::default() }),
        ("1;4", Style { bold: true, underline: true, ..Style::default() }),
        ("2;3;9;7", Style { dim: true, italic: true, strike: true, inverse: true, ..Style::default() }),
        ("38;5;9", Style { fg: Some(Colour::Palette(9)), ..Style::default() }),
        ("38;5;196", Style { fg: Some(Colour::Rgb(255, 0, 0)), ..Style::default() }),
        ("38;5;232", Style { fg: Some(Colour::Rgb(8, 8, 8)), ..Style::default() }),
        ("48;2;10;20;30", Style { bg: Some(Colour::Rgb(10, 20, 30)), ..Style::default() }),
    ];
    for (codes, expected) in cases {
        let spans = parse(&format!("\x1b[{codes}mx"));
        assert_eq!(spans[0].style, expected, "codes {codes}");
    }
}

#[test]
fn sgr_resets_return_to_plain() {
    for input in ["\x1b[1;31ma\x1b[0mb", "\x1b[1;31ma\x1b[mb", "\x1b[1;31ma\x1b[22;39mb"] {
        let spans = parse(input);
        assert_eq!(spans.len(), 2, "{input:?}");
        assert!(!spans[0].style.is_plain());
        assert!(spans[1].style.is_plain());
    }
}

#[test]
fn other_escape_sequences_and_control_characters_are_removed() {
    let cases = [
        ("\x1b[2Kcleared", "cleared"),
        ("\x1b[1A\x1b[10Gmoved", "moved"),
        ("\x1b]0;window title\x07shown", "shown"),
        ("\x1b]8;;https://example.com\x1b\\link\x1b]8;;\x1b\\", "link"),
        ("bell\x07 and\x08 backspace", "bell and backspace"),
        ("tab\tkept", "tab\tkept"),
        ("truncated \x1b[31", "truncated "),
        ("lone escape \x1b", "lone escape "),
    ];
    for (input, expected) in cases {
        assert_eq!(parsed_text(input), expected, "{input:?}");
    }
}

#[test]
fn carriage_returns_redraw_the_line() {
    let cases = [
        ("one\r\ntwo\r\n", "one\ntwo\n"),
        ("done\n 10%\r 50%\r100%\nnext", "done\n100%\nnext"),
        ("\x1b[32m 10%\r\x1b[31m 90%", " 90%"),
    ];
    for (input, expected) in cases {
        assert_eq!(parsed_text(input), expected, "{input:?}");
    }
}
