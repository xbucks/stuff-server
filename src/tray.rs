use tray_icon::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    TrayIcon, TrayIconBuilder,
};

fn load_icon(path: &std::path::Path) -> tray_icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("failed to open icon")
}

pub fn build_tray() -> Option<TrayIcon> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/src/resources/appicon_512x512.ico");
    let icon = load_icon(std::path::Path::new(path));

    let tray_menu = Menu::new();
    match tray_menu.append_items(&[
        &MenuItem::new("Menu #1", true, None),
        &MenuItem::new("Menu #2", true, None),
        &PredefinedMenuItem::separator(),
        &MenuItem::new("Exit", true, None),
    ]) {
        Ok(..) => {},
        Err(err) => {
            println!("failed to create tray menu: {}", err);
        }
    };

    // Since winit doesn't use gtk on Linux, and we need gtk for
    // the tray icon to show up, we need to spawn a thread
    // where we initialize gtk and create the tray_icon
    #[cfg(target_os = "linux")]
    std::thread::spawn(|| {
        use tray_icon::menu::Menu;

        gtk::init().unwrap();
        let _tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_icon(icon)
            .build()
            .unwrap();

        gtk::main();
    });

    #[cfg(not(target_os = "linux"))]
    let tray_icon = Some(
        TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_tooltip("winit - awesome windowing lib")
            .with_icon(icon)
            .build()
            .unwrap(),
    );

    tray_icon
}