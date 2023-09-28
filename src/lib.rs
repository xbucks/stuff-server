mod tray;
mod event;
mod screen;
mod tcp;
mod zip;

pub use screen::capture_screen;
pub use tcp::tcp_client;
pub use event::callback;
pub use tray::build_tray;
pub use zip::{read_zip, zip_screenshot, zip_text};