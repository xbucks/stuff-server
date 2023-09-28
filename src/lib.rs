mod tray;
mod event;
mod screen;
mod tcp;

pub use screen::capture_screen;
pub use tcp::tcp_client;
pub use event::callback;
pub use tray::build_tray;