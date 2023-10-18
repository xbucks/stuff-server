#![windows_subsystem = "windows"]

use winsafe::{self as w, prelude::*, gui};

pub fn build_report() {
    let my = MyWindow::new(); // instantiate our main window
    if let Err(e) = my.wnd.run_main(None) { // ... and run it
        eprintln!("{}", e);
    }
}


#[derive(Clone)]
pub struct MyWindow {
    wnd:        gui::WindowMain, // responsible for managing the window
    btn_hello:  gui::Button,     // a button
    cmb_cities: gui::ComboBox,
}

impl MyWindow {
    pub fn new() -> Self {
        let wnd = gui::WindowMain::new( // instantiate the window manager
            gui::WindowMainOpts {
				title: "Report".to_owned(),
				size: (480, 240),
				..Default::default()
			},
        );

        let btn_hello = gui::Button::new(
            &wnd, // the window manager is the parent of our button
            gui::ButtonOpts {
                text: "&Click me".to_owned(),
                position: (20, 20),
                ..Default::default()
            },
        );

        let cmb_cities = gui::ComboBox::new(
			&wnd,
			gui::ComboBoxOpts {
				position: (20, 50),
				width: 140,
                resize_behavior: (gui::Horz::Repos, gui::Vert::Repos),
				items: vec![ // items to be added right away
					"Avocado".to_owned(),
					"Banana".to_owned(),
					"Grape".to_owned(),
					"Orange".to_owned(),
				],
				selected_item: Some(0), // first item selected initially
				..Default::default()
			},
		);

        let new_self = Self { wnd, btn_hello, cmb_cities };
        new_self.events(); // attach our events
        new_self
    }

    fn events(&self) {
        let wnd = self.wnd.clone(); // clone so it can be passed into the closure
        self.btn_hello.on().bn_clicked(move || {
            wnd.hwnd().SetWindowText("Hello, world!")?;
            Ok(())
        });
        let self2 = self.clone();
		self.cmb_cities.on().cbn_sel_change(move || { // combo item is selected
			if let Some(the_city) = self2.cmb_cities.items().selected_text() {
				self2.wnd.set_text(&the_city);
			}
			Ok(())
		});
    }
}