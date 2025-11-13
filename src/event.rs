use crossterm::event::KeyEvent;

pub enum GuiEvent {

    Input(KeyEvent),
    Tick,
}