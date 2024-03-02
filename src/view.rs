use crate::model::Model;
use unicode_segmentation::UnicodeSegmentation;

use ratatui::{prelude::*, widgets::*};

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
                Constraint::Fill(5),
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(5),
            ])
            .split(main_layout[1]);
        let middle_lower_layouts = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Fill(1), Constraint::Fill(1)])
            .split(middle_layouts[3]);

        let right_layouts = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ])
            .split(main_layout[2]);

        if self.config.show_frame_statistics {
            let average_frametime = self.frame_statistics.average_frame_duration.as_secs_f64();
            let average_fps = 1.0 / average_frametime;

            frame.render_widget(
                Paragraph::new(format!(
                    "Frametime {:.0}ms ({:.0}FPS)",
                    average_frametime * 1_000.0,
                    average_fps
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
                Paragraph::new(Line::from(spans)).wrap(Wrap { trim: false }),
                middle_layouts[2],
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
                        Span::styled("Enter", shortcut_style),
                        Span::from(" - "),
                        Span::styled("next test", action_style),
                    ]),
                    Line::from(vec![
                        Span::styled("Control-l", shortcut_style),
                        Span::from(" - "),
                        Span::styled("toggle live typing statistics", action_style),
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

        if self.config.show_live_typing_statistics && self.current_test.is_started() {
            let accuracy_style = {
                let accuracy = self.current_test.accuracy();
                if accuracy >= 1.0 {
                    Style::default().fg(Color::LightGreen)
                } else if accuracy >= 0.95 {
                    Style::default().fg(Color::Green)
                } else if accuracy >= 0.75 {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default().fg(Color::Red)
                }
            };

            frame.render_widget(
                Gauge::default()
                    .gauge_style(
                        Style::default()
                            .fg(accuracy_style.fg.unwrap())
                            .bg(Color::Black),
                    )
                    .use_unicode(true)
                    .ratio(self.current_test.completion()),
                middle_layouts[4].clamp(Rect::new(
                    middle_layouts[4].x,
                    middle_layouts[4].y,
                    middle_layouts[4].width,
                    1,
                )),
            );

            frame.render_widget(
                Paragraph::new(vec![
                    Line::from(vec![Span::styled(
                        format!("Accuracy: {:.2}%", self.current_test.accuracy() * 100.0),
                        accuracy_style,
                    )]),
                    Line::from(vec![Span::styled(
                        format!("WPM: {:.0}", self.current_test.wpm()),
                        accuracy_style,
                    )]),
                    Line::from(vec![Span::styled(
                        format!("Raw WPM: {:.0}", self.current_test.raw_wpm()),
                        accuracy_style,
                    )]),
                    Line::from(vec![Span::styled(
                        format!("Duration: {}s", self.current_test.duration().as_secs()),
                        accuracy_style,
                    )]),
                ]),
                middle_lower_layouts[1],
            );
        }
    }
}
