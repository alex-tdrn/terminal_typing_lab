use crate::action::Action;

use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

pub struct Model {
    pub frame_statistics: FrameStatistics,
    pub config: Config,
    pub should_quit: bool,
    pub current_test: Test,
}

impl Model {
    pub fn update(&mut self, action: Action) {
        match action {
            Action::CharacterInput(c) => self.current_test.input(c),
            Action::DeleteCharacter => self.current_test.delete_character(),
            Action::DeleteWord => {
                self.current_test.delete_word();
            },
            Action::Restart => self.current_test = Test::new("Lorem ä ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."),
            Action::ToggleStatistics => {
                self.config.show_frame_statistics = !self.config.show_frame_statistics
            }
            Action::Quit => self.should_quit = true,
        }
    }
}

impl Default for Model {
    fn default() -> Self {
        Model {
            frame_statistics: FrameStatistics::default(),
            config: Config::default(),
            should_quit: false,
            current_test: Test::new("Lorem ä ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."),
        }
    }
}

pub struct Config {
    pub show_frame_statistics: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            show_frame_statistics: true,
        }
    }
}

pub struct FrameStatistics {
    pub frame_begin: std::time::Instant,
    pub last_frame_duration: std::time::Duration,
    pub average_frame_duration: std::time::Duration,
}

impl FrameStatistics {
    pub fn new_frame(&mut self) {
        let new_frame_begin = std::time::Instant::now();
        self.last_frame_duration = new_frame_begin - self.frame_begin;
        self.frame_begin = new_frame_begin;
        self.average_frame_duration =
            self.average_frame_duration.mul_f64(0.8) + self.last_frame_duration.mul_f64(0.2);
    }
}

impl Default for FrameStatistics {
    fn default() -> Self {
        FrameStatistics {
            frame_begin: std::time::Instant::now(),
            last_frame_duration: std::time::Duration::default(),
            average_frame_duration: std::time::Duration::default(),
        }
    }
}

pub struct Test {
    pub target_text: String,
    pub target_text_grapheme_count: usize,
    pub current_text: String,
    pub current_text_grapheme_count: usize,
    pub start_time: Option<std::time::Instant>,
    pub end_time: Option<std::time::Instant>,
}

impl Test {
    pub fn new(target_text: &str) -> Self {
        let normalized_target_text = target_text.nfc().to_string();
        let grapheme_count = normalized_target_text.graphemes(true).count();

        Test {
            target_text: normalized_target_text,
            target_text_grapheme_count: grapheme_count,
            current_text: String::default(),
            current_text_grapheme_count: 0,
            start_time: None,
            end_time: None,
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(std::time::Instant::now());
    }

    pub fn is_started(&self) -> bool {
        self.start_time.is_some()
    }

    pub fn finish(&mut self) {
        self.end_time = Some(std::time::Instant::now());
    }

    pub fn is_finished(&self) -> bool {
        self.end_time.is_some()
    }

    pub fn restart(&mut self) {
        *self = Test::new(self.target_text.as_str());
    }

    pub fn input(&mut self, c: char) {
        if self.is_finished() {
            return;
        }

        if !self.is_started() {
            self.start();
        }

        self.current_text.push(c);
        self.normalize_current_text();

        if self.completion() >= 1.0 {
            self.finish();
        }
    }

    pub fn delete_character(&mut self) {
        if !self.is_finished() {
            if let Some((byte_offset, _)) = self.current_text.grapheme_indices(true).last() {
                self.current_text.truncate(byte_offset);
                self.normalize_current_text();
            }
        }
    }

    pub fn delete_word(&mut self) {
        if !self.is_finished() {
            if let Some((byte_offset, _)) = self.current_text.unicode_word_indices().last() {
                self.current_text.truncate(byte_offset);
                self.normalize_current_text();
            }
        }
    }

    pub fn correct_graphemes(&self) -> usize {
        std::iter::zip(
            self.target_text.graphemes(true),
            self.current_text.graphemes(true),
        )
        .filter(|(target, current)| target == current)
        .count()
    }

    pub fn accuracy(&self) -> f64 {
        self.correct_graphemes() as f64 / self.current_text_grapheme_count as f64
    }

    pub fn wpm(&self) -> f64 {
        todo!()
    }

    pub fn raw_wpm(&self) -> f64 {
        todo!()
    }

    pub fn completion(&self) -> f64 {
        self.current_text_grapheme_count as f64 / self.target_text_grapheme_count as f64
    }

    fn normalize_current_text(&mut self) {
        self.current_text = self.current_text.nfc().to_string();
        self.current_text_grapheme_count = self.current_text.graphemes(true).count();
    }
}
