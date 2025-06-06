use ratatui::layout::{Constraint, Direction, Layout, Rect, Size};
use ratatui::style::Style;
use ratatui::text::{Line, Span};

use crate::constants::{MAC_CYAN_COLOR, MAC_PURPLE_COLOR, PLASTIC_PRIMARY_COLOR};

pub fn _center(area: Rect, max_size: Size) -> Rect {
    let horizontal_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Fill(1),
            Constraint::Max(max_size.width),
            Constraint::Fill(1),
        ],
    );
    let vertical_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Fill(1),
            Constraint::Max(max_size.height),
            Constraint::Fill(1),
        ],
    );
    let [_, horizontal_center, _] = *horizontal_layout.split(area) else {
        unreachable!("length is always three")
    };
    let [_, center, _] = *vertical_layout.split(horizontal_center) else {
        unreachable!("length is always three")
    };

    center
}

pub fn interpolate_and_truncate(
    source: &str,
    max_characters: usize,
    recipients: &str,
    time_limit: &str,
) -> Line<'static> {
    let mut spans = Vec::new();
    let mut current_position = 0;
    let mut display_characters = 0;
    let mut characters = source.char_indices().peekable();

    while let Some((index, character)) = characters.next() {
        if display_characters >= max_characters {
            break;
        }

        if character == '{' {
            if index > current_position {
                let text = &source[current_position..index];
                let text_characters = text.chars().count();

                if display_characters + text_characters <= max_characters {
                    spans.push(Span::raw(text.to_string()));
                    display_characters += text_characters;
                } else {
                    let remaining = max_characters - display_characters;
                    let truncated: String = text.chars().take(remaining).collect();
                    spans.push(Span::raw(truncated));
                    display_characters = max_characters;
                    break;
                }
            }

            let start = index;
            let mut found_closing = false;
            let mut keyword_end = index;

            for (idx, ch) in characters.by_ref() {
                if ch == '}' {
                    keyword_end = idx;
                    found_closing = true;
                    break;
                }
            }

            if found_closing {
                let keyword = &source[start + 1..keyword_end];

                let display_text = match keyword {
                    "recipients" => recipients.to_string(),
                    "time_limit" => time_limit.to_string(),
                    _ => format!("{{{}}}", keyword),
                };

                let display_characters_count = display_text.chars().count();

                if display_characters + display_characters_count <= max_characters {
                    let color = match keyword {
                        "recipients" => MAC_PURPLE_COLOR,
                        "time_limit" => MAC_CYAN_COLOR,
                        _ => PLASTIC_PRIMARY_COLOR,
                    };

                    spans.push(Span::styled(display_text, Style::default().fg(color)));
                    display_characters += display_characters_count;
                    current_position = keyword_end + 1;
                } else {
                    break;
                }
            }
        }
    }

    if current_position < source.len() && display_characters < max_characters {
        let remaining_text = &source[current_position..];
        let remaining_chars = remaining_text.chars().count();

        if display_characters + remaining_chars <= max_characters {
            spans.push(Span::raw(remaining_text.to_string()));
        } else {
            let chars_to_take = max_characters - display_characters;
            let truncated: String = remaining_text.chars().take(chars_to_take).collect();
            spans.push(Span::raw(truncated));
        }
    }

    Line::from(spans)
}
