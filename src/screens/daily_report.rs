use nwd::{NwgUi, NwgPartial};
use nwg::NativeUi;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::LineWriter;
use std::io::{Write, BufReader, BufRead, Error};
use crate::zip_report;
use crate::REPORT;

pub fn build_daily() {
    nwg::init().expect("Failed to init Native Windows GUI");
    //nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let mut font = nwg::Font::default();
    nwg::Font::builder()
        .family("MS Shell Dlg")
        .size(15)
        .build(&mut font)
        .expect("Failed to build font");
    nwg::Font::set_global_default(Some(font));

    let _ui = ConfigDlg::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}

#[derive(Default, NwgUi)]
pub struct ConfigDlg {
    #[nwg_control(size: (900, 600), position: (100, 100), title: "Daily Report")]
    #[nwg_events(OnInit: [ConfigDlg::init], OnResize: [ConfigDlg::size], OnWindowClose: [ConfigDlg::exit])]
    window: nwg::Window,

    #[nwg_layout(parent: window)]
    layout: nwg::DynLayout,

    #[nwg_control(position: (20, 30), size: (600, 530), collection: vec![])]
    list: nwg::ListBox<String>,

    #[nwg_control(text: "Cancel", position: (560, 550), size: (100, 25))]
    #[nwg_events(OnButtonClick: [ConfigDlg::cancel])]
    cancel_btn: nwg::Button,

    #[nwg_control(text: "Ok", position: (670, 550), size: (100, 25))]
    #[nwg_events(OnButtonClick: [ConfigDlg::ok])]
    ok_btn: nwg::Button,

    #[nwg_control(text: "Submit", position: (780, 550), size: (100, 25))]
    #[nwg_events(OnButtonClick: [ConfigDlg::submit])]
    submit_btn: nwg::Button,

    #[nwg_control(position: (640, 30), size: (240, 510))]
    frame: nwg::Frame,

    #[nwg_partial(parent: frame)]
    #[nwg_events((save_btn, OnButtonClick): [ConfigDlg::save], (type_cbx, OnComboxBoxSelection): [ConfigDlg::on_type])]
    controls: Controls,
}

impl ConfigDlg {
    fn init(&self) {
        self.frame.set_visible(true);

        self.layout.add_child((0, 0), (50, 100), &self.list);
        self.layout.add_child((100, 100), (0, 0), &self.ok_btn);
        self.layout.add_child((100, 100), (0, 0), &self.cancel_btn);
        self.layout.add_child((100, 100), (0, 0), &self.submit_btn);

        self.layout.add_child((50, 0), (50, 100), &self.frame);

        self.controls.init(&self.frame);

        self.layout.fit();

        Self::read_report(&self);
    }

    fn size(&self) {
        self.layout.fit();
    }

    fn on_type(&self) {
        let mut items = self.controls.type_cbx.collection_mut();
        let index = self.controls.type_cbx.selection().unwrap_or(0);
        println!("{:?}", items.get(index).unwrap());
    }

    fn save(&self) {
        let items_when = self.controls.when_cbx.collection_mut();
        let index_when = self.controls.when_cbx.selection().unwrap_or(0);
        let _when = items_when.get(index_when).unwrap();

        let items_type = self.controls.type_cbx.collection_mut();
        let index_type = self.controls.type_cbx.selection().unwrap_or(0);
        let _type = items_type.get(index_type).unwrap();

        let items_where = self.controls.where_cbx.collection_mut();
        let index_where = self.controls.where_cbx.selection().unwrap_or(0);
        let _where = items_where.get(index_where).unwrap();

        let _amount = self.controls.amount_input.text();
        let _hours = self.controls.hours_input.text();
        let _note = self.controls.note_box.text();

        let text: String = format!("{}|{}|{}|{}|{}|{}", _when, _type, _where, _amount, _hours, _note);
        let display = format!("- {} | {} | {} | {} | {} hours | {}", _when, _type, _where, _amount, _hours, _note);
        let _text = text.clone();
        let _display = display.clone();

        self.list.push(_display);
        REPORT.lock().unwrap().push(_text);
    }

    fn read_report(&self) -> Result<(), Error> {
        let path = "report.txt";
    
        let input = File::open(path)?;
        let buffered = BufReader::new(input);
    
        for line in buffered.lines() {
            let _line = line.unwrap().clone();
            let parts: Vec<&str> = _line.split("|").collect();
            let display = format!("- {} | {} | {} | {} | {} hours | {}", parts[0], parts[1], parts[2], parts[3], parts[4], parts[5]);
            self.list.push(display);
            REPORT.lock().unwrap().push(_line);
        }
    
        Ok(())
    }

    fn write_report() -> Result<(), Error> {
        let file = File::create("report.txt")?;
        let mut file = LineWriter::new(file);

        for name in REPORT.lock().unwrap().iter() {
            file.write_all(name.as_bytes());
            file.write_all(b"\n")?;
        }

        file.flush()?;

        Ok(())
    }

    fn ok(&self) {
        match Self::write_report() {
            Ok(..) => (),
            Err(..) => println!("failed to save daily report.")
        };
    }

    fn cancel(&self) {
        nwg::stop_thread_dispatch();
    }

    fn submit(&self) {
        match Self::write_report() {
            Ok(..) => (),
            Err(..) => println!("failed to submit daily report.")
        };
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

#[derive(Default, NwgPartial)]
pub struct Controls {
    #[nwg_layout]
    layout: nwg::DynLayout,

    #[nwg_control(text: "When:", h_align: HTextAlign::Right, position: (10, 10), size: (100, 20))]
    label1: nwg::Label,
    #[nwg_control(text: "What:", h_align: HTextAlign::Right, position: (10, 40), size: (100, 20))]
    label2: nwg::Label,
    #[nwg_control(text: "Where:", h_align: HTextAlign::Right, position: (10, 70), size: (100, 20))]
    label3: nwg::Label,
    #[nwg_control(text: "Amount:", h_align: HTextAlign::Right, position: (10, 100), size: (100, 20))]
    label4: nwg::Label,
    #[nwg_control(text: "Hours:", h_align: HTextAlign::Right, position: (10, 130), size: (100, 20))]
    label5: nwg::Label,
    #[nwg_control(text: "Note:", h_align: HTextAlign::Right, position: (10, 160), size: (100, 20))]
    label6: nwg::Label,

    #[nwg_control(collection: vec!["Morning", "Afternoon", "Evening"], position: (120, 10), size: (100, 20), selected_index: Some(0))]
    when_cbx: nwg::ComboBox<&'static str>,

    #[nwg_control(collection: vec!["Bid", "Develop", "Interview/Chat", "Interview/Intro Call", "Interview/Tech Call", "Interview/CEO Call", "Others"], position: (120, 40), size: (100, 20), selected_index: Some(0))]
    type_cbx: nwg::ComboBox<&'static str>,

    #[nwg_control(collection: vec!["Upwork/Real", "Upwork/Fake", "Freelancer", "LinkedIn", "Lancers", "Crowd", "Doda", "WellFound", "Telegram", "Discord", "Others"], position: (120, 70), size: (100, 20), selected_index: Some(0))]
    where_cbx: nwg::ComboBox<&'static str>,

    #[nwg_control(text: "1", flags: "NUMBER|VISIBLE", position: (120, 100), size: (100, 20))]
    amount_input: nwg::TextInput,

    #[nwg_control(text: "3", flags: "NUMBER|VISIBLE", position: (120, 130), size: (100, 20))]
    hours_input: nwg::TextInput,

    #[nwg_control(text: "Please put your note here!", position: (120, 160), size: (133, 200))]
    note_box: nwg::TextBox,

    #[nwg_control(text: "Save", position: (20, 470), size: (60, 25))]
    save_btn: nwg::Button,

    #[nwg_control(text: "Edit", position: (90, 470), size: (60, 25))]
    edit_btn: nwg::Button,

    #[nwg_control(text: "Delete", position: (160, 470), size: (60, 25))]
    delete_btn: nwg::Button,
}

impl Controls {
    fn init(&self, frame: &nwg::Frame) {
        self.layout.parent(frame);

        self.layout.add_child((0, 0), (0, 0), &self.label1);
        self.layout.add_child((0, 0), (0, 0), &self.label2);
        self.layout.add_child((0, 0), (0, 0), &self.label3);
        self.layout.add_child((0, 0), (0, 0), &self.label4);
        self.layout.add_child((0, 0), (0, 0), &self.label5);
        self.layout.add_child((0, 0), (0, 0), &self.label6);

        self.layout.add_child((0, 0), (100, 0), &self.when_cbx);
        self.layout.add_child((0, 0), (100, 0), &self.type_cbx);
        self.layout.add_child((0, 0), (100, 0), &self.where_cbx);
        self.layout.add_child((0, 0), (100, 0), &self.amount_input);
        self.layout.add_child((0, 0), (100, 0), &self.hours_input);
        self.layout.add_child((0, 0), (100, 0), &self.note_box);

        self.layout.add_child((0, 100), (0, 0), &self.save_btn);
        self.layout.add_child((0, 100), (0, 0), &self.edit_btn);
        self.layout.add_child((0, 100), (0, 0), &self.delete_btn);
    }
}