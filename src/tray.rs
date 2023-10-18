use std::sync::mpsc::{self, Sender, Receiver};
use trayicon::*;
use crate::Events;

pub fn build_tray() -> (TrayIcon<Events>, Receiver<Events>) {
    let (s, r) = mpsc::channel::<Events>();
    let icon = include_bytes!("./resources/appicon_512x512.ico");

    // Needlessly complicated tray icon with all the whistles and bells
    let tray_icon = TrayIconBuilder::new()
        .sender(s)
        .icon_from_buffer(icon)
        .tooltip("Cool Tray 👀 Icon")
        .on_click(Events::ClickTrayIcon)
        .on_double_click(Events::DoubleClickTrayIcon)
        .menu(
            MenuBuilder::new()
                .item("Report", Events::Item1)
                .item("Item 2", Events::Item2)
                .item("Item 3", Events::Item3)
                .separator()
                .item("E&xit", Events::Exit),
        )
        .build()
        .unwrap();
    (tray_icon, r)
}