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
            }
            Action::Restart => self.current_test.restart(),
            Action::NextTest => self.next_test(),
            Action::ToggleFrameStatistics => {
                self.config.show_frame_statistics = !self.config.show_frame_statistics
            }
            Action::ToggleLiveTypingStatistics => {
                self.config.show_live_typing_statistics = !self.config.show_live_typing_statistics
            }
            Action::Quit => self.should_quit = true,
        }
    }

    fn next_test(&mut self) {
        self.current_test = Test::new("What the fuck did you just fucking say about me, you little bitch? I'll have you know I graduated top of my class in the Navy Seals, and I've been involved in numerous secret raids on Al-Quaeda, and I have over 300 confirmed kills. I am trained in gorilla warfare and I'm the top sniper in the entire US armed forces. You are nothing to me but just another target. I will wipe you the fuck out with precision the likes of which has never been seen before on this Earth, mark my fucking words. You think you can get away with saying that shit to me over the Internet? Think again, fucker. As we speak I am contacting my secret network of spies across the USA and your IP is being traced right now so you better prepare for the storm, maggot. The storm that wipes out the pathetic little thing you call your life. You're fucking dead, kid. I can be anywhere, anytime, and I can kill you in over seven hundred ways, and that's just with my bare hands. Not only am I extensively trained in unarmed combat, but I have access to the entire arsenal of the United States Marine Corps and I will use it to its full extent to wipe your miserable ass off the face of the continent, you little shit. If only you could have known what unholy retribution your little 'clever' comment was about to bring down upon you, maybe you would have held your fucking tongue. But you couldn't, you didn't, and now you're paying the price, you goddamn idiot. I will shit fury all over you and you will drown in it. You're fucking dead, kiddo");
    }
}

impl Default for Model {
    fn default() -> Self {
        Model {
            frame_statistics: FrameStatistics::default(),
            config: Config::default(),
            should_quit: false,
            current_test: Test::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."),
        }
    }
}

pub struct Config {
    pub show_frame_statistics: bool,
    pub show_live_typing_statistics: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            show_frame_statistics: true,
            show_live_typing_statistics: true,
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
        if self.current_text_grapheme_count == 0 {
            0.0
        } else {
            self.correct_graphemes() as f64 / self.current_text_grapheme_count as f64
        }
    }

    pub fn duration(&self) -> std::time::Duration {
        if let Some(start_time) = self.start_time {
            self.end_time
                .unwrap_or(std::time::Instant::now())
                .duration_since(start_time)
        } else {
            std::time::Duration::default()
        }
    }

    pub fn wpm(&self) -> f64 {
        self.calculate_wpm(self.correct_graphemes())
    }

    pub fn raw_wpm(&self) -> f64 {
        self.calculate_wpm(self.current_text_grapheme_count)
    }

    pub fn completion(&self) -> f64 {
        self.current_text_grapheme_count as f64 / self.target_text_grapheme_count as f64
    }

    fn normalize_current_text(&mut self) {
        self.current_text = self.current_text.nfc().to_string();
        self.current_text_grapheme_count = self.current_text.graphemes(true).count();
    }

    fn calculate_wpm(&self, grapheme_count: usize) -> f64 {
        let duration_in_seconds = self.duration().as_secs_f64();
        if duration_in_seconds > 0.0 {
            let correct_words = grapheme_count as f64 / 5.0;
            correct_words * 60.0 / duration_in_seconds
        } else {
            0.0
        }
    }
}
