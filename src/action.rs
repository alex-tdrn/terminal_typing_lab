pub enum Action {
    ToggleFrameStatistics,
    ToggleLiveTypingStatistics,
    CharacterInput(char),
    DeleteCharacter,
    DeleteWord,
    Restart,
    NextTest,
    Quit,
}
