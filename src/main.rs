pub mod action;
pub mod model;
pub mod terminal;
pub mod view;

use model::Model;

use terminal::*;

fn main() -> std::io::Result<()> {
    let mut model = Model::default();

    let mut terminal = create_terminal()?;

    while !model.should_quit {
        model.frame_statistics.new_frame();

        terminal.draw(|frame| {
            model.view(frame);
        })?;

        if let Some(action) = poll_terminal() {
            model.update(action);
        }
    }

    destroy_terminal()?;

    Ok(())
}
