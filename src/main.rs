#![allow(non_snake_case)]
mod ui;
mod work;

use std::{
    env,
    error::Error,
    sync::{Arc, Mutex},
    thread,
};
use ui::files::FilesState;
use work::start::start as workStart;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

// Main thread to work on UI rendering
fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    // Global State that is Shared Across Threads
    let file_state = Arc::new(Mutex::new(FilesState::new()));

    // Spawn worker thread
    let file_state_working_thread = file_state.clone();

    let handle = thread::spawn(move || workStart(file_state_working_thread, &args[0]));
    handle.join().unwrap();
    // Draw the UI
    ui::ui::draw_ui(file_state)?;
    Ok(())
}
