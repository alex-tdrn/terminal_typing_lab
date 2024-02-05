use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    ExecutableCommand,
};

use crate::action::Action;

pub fn create_terminal(
) -> std::io::Result<ratatui::prelude::Terminal<ratatui::prelude::CrosstermBackend<std::io::Stdout>>>
{
    std::io::stdout().execute(crossterm::terminal::EnterAlternateScreen)?;
    crossterm::terminal::enable_raw_mode()?;

    let mut terminal = ratatui::prelude::Terminal::new(ratatui::prelude::CrosstermBackend::new(
        std::io::stdout(),
    ))?;

    terminal.clear()?;

    Ok(terminal)
}

pub fn poll_terminal() -> Option<Action> {
    let Ok(true) = event::poll(std::time::Duration::from_millis(1)) else {
        return None;
    };

    let Ok(Event::Key(KeyEvent {
        code,
        modifiers,
        kind: KeyEventKind::Press,
        state: _,
    })) = event::read()
    else {
        return None;
    };

    match (modifiers, code) {
        (KeyModifiers::NONE, KeyCode::Esc)
        | (KeyModifiers::CONTROL, KeyCode::Char('c'))
        | (KeyModifiers::CONTROL, KeyCode::Char('q')) => Some(Action::Quit),
        (KeyModifiers::CONTROL, KeyCode::Char('s')) => Some(Action::ToggleStatistics),
        (KeyModifiers::NONE, KeyCode::Tab) => Some(Action::Restart),
        (KeyModifiers::NONE, KeyCode::Backspace) | (KeyModifiers::CONTROL, KeyCode::Char('h')) => {
            Some(Action::DeleteCharacter)
        }
        (KeyModifiers::CONTROL, KeyCode::Backspace)
        | (KeyModifiers::CONTROL, KeyCode::Char('w')) => Some(Action::DeleteWord),
        (KeyModifiers::NONE, KeyCode::Char(c)) | (KeyModifiers::SHIFT, KeyCode::Char(c)) => {
            Some(Action::CharacterInput(c))
        }
        _ => None,
    }
}

pub fn destroy_terminal() -> std::io::Result<()> {
    std::io::stdout().execute(crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
