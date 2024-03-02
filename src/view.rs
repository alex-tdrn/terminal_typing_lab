use crate::model::Model;
use unicode_segmentation::UnicodeSegmentation;

use ratatui::{prelude::*, style::*, widgets::*};

impl Model {
    pub fn view(&self, frame: &mut ratatui::Frame) {
        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ])
            .split(frame.size());

        let middle_layouts = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(2),
                Constraint::Fill(1),
                Constraint::Fill(2),
            ])
            .split(main_layout[1]);
        let middle_lower_layouts = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Fill(1), Constraint::Fill(1)])
            .split(middle_layouts[2]);

        let right_layouts = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ])
            .split(main_layout[2]);

        if self.config.show_frame_statistics {
            let average_frametime =
                std::cmp::max(self.frame_statistics.average_frame_duration.as_millis(), 1);
            let average_fps = 1_000 / average_frametime;

            frame.render_widget(
                Paragraph::new(format!(
                    "Frametime {}ms ({}FPS)",
                    average_frametime, average_fps
                ))
                .centered(),
                right_layouts[1],
            );
        }

        {
            let target_char_style = Style::default().fg(Color::DarkGray);
            let correct_char_style = Style::default().fg(Color::LightGreen);
            let incorrect_char_style = Style::default().fg(Color::LightRed).underlined();

            let mut spans = Vec::new();

            for (target, current) in std::iter::zip(
                self.current_test.target_text.graphemes(true),
                self.current_test.current_text.graphemes(true),
            ) {
                if target == current {
                    spans.push(Span::styled(current.to_string(), correct_char_style));
                } else {
                    spans.push(Span::styled(current.to_string(), incorrect_char_style));
                }
            }

            for remaining in self
                .current_test
                .target_text
                .graphemes(true)
                .skip(self.current_test.current_text_grapheme_count)
            {
                spans.push(Span::styled(remaining.to_string(), target_char_style));
            }

            frame.render_widget(
                Paragraph::new(Line::from(spans)).wrap(Wrap { trim: true }),
                middle_layouts[1],
            );
        }

        {
            let shortcut_style = Style::default().fg(Color::Yellow);
            let action_style = Style::default().fg(Color::Blue);

            frame.render_widget(
                Paragraph::new(vec![
                    Line::from(vec![
                        Span::styled("Backspace", shortcut_style),
                        Span::from(", or "),
                        Span::styled("Control-h", shortcut_style),
                        Span::from(" - "),
                        Span::styled("delete character", action_style),
                    ]),
                    Line::from(vec![
                        Span::styled("Control-Backspace", shortcut_style),
                        Span::from(", or "),
                        Span::styled("Control-w", shortcut_style),
                        Span::from(" - "),
                        Span::styled("delete word", action_style),
                    ]),
                    Line::from(vec![
                        Span::styled("Tab", shortcut_style),
                        Span::from(" - "),
                        Span::styled("restart", action_style),
                    ]),
                    Line::from(vec![
                        Span::styled("Control-s", shortcut_style),
                        Span::from(" - "),
                        Span::styled("toggle frame statistics", action_style),
                    ]),
                    Line::from(vec![
                        Span::styled("Esc", shortcut_style),
                        Span::from(", "),
                        Span::styled("Control-c", shortcut_style),
                        Span::from(", or "),
                        Span::styled("Control-q", shortcut_style),
                        Span::from(" - "),
                        Span::styled("quit", action_style),
                    ]),
                ]),
                middle_lower_layouts[0],
            );
        }

        {
            let accuracy_style = {
                let accuracy = self.current_test.accuracy();
                if accuracy >= 1.0 {
                    Style::default().fg(Color::Green)
                } else if accuracy >= 0.75 {
                    Style::default().fg(Color::Blue)
                } else if accuracy >= 0.5 {
                    Style::default().fg(Color::Yellow)
                } else if accuracy >= 0.25 {
                    Style::default().fg(Color::Red)
                } else {
                    Style::default().fg(Color::Gray)
                }
            };

            frame.render_widget(
                Paragraph::new(vec![
                    Line::from(vec![Span::styled(
                        format!("Accuracy:{}", self.current_test.accuracy()),
                        accuracy_style,
                    )]),
                    Line::from(vec![Span::styled(
                        format!("WPM:{}", self.current_test.wpm()),
                        accuracy_style,
                    )]),
                    Line::from(vec![Span::styled(
                        format!("Raw WPM:{}", self.current_test.raw_wpm()),
                        accuracy_style,
                    )]),
                ]),
                middle_lower_layouts[1],
            );
        }
    }
}
