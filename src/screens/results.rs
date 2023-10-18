use nwd::NwgUi;
use nwg::NativeUi;

pub fn build_results() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}

#[derive(Default, NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (800, 600), position: (200, 200), title: "Report", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [BasicApp::say_goodbye] )]
    window: nwg::Window,

    #[nwg_control(collection: vec!["Morning", "Afternoon", "Evening"], size: (100, 25), position: (10, 10), selected_index: Some(0))]
    ctl_when: nwg::ComboBox<&'static str>,

    #[nwg_control(collection: vec!["Bid", "Develop", "Chat"], size: (80, 25), position: (120, 10), selected_index: Some(0))]
    #[nwg_events( OnComboxBoxSelection: [BasicApp::on_type_changed] )]
    ctl_type: nwg::ComboBox<&'static str>,

    #[nwg_control(collection: vec!["Upwork", "Freelancer", "LinkedIn", "Lancers", "Crowd", "Discord", "Telegram", "Others"], size: (100, 25), position: (210, 10), selected_index: Some(0))]
    ctl_where: nwg::ComboBox<&'static str>,

    #[nwg_control(text: "Amount", size: (60, 25), position: (320, 10))]
    ctl_amount: nwg::TextInput,

    #[nwg_control(text: "Hours", size: (60, 25), position: (390, 10))]
    ctl_hours: nwg::TextInput,

    #[nwg_control(text: "+", size: (40, 25), position: (650, 10))]
    #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    btn_add: nwg::Button,

    #[nwg_control(text: "-", size: (40, 25), position: (700, 10))]
    #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    btn_delete: nwg::Button,

    #[nwg_control(text: "?", size: (40, 25), position: (750, 10))]
    #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    btn_edit: nwg::Button,

    #[nwg_control(text: "Description", size: (460, 160), position: (10, 50))]
    ctl_note: nwg::TextBox,
}

impl BasicApp {
    fn on_type_changed(&self) {

    }

    fn say_hello(&self) {
        nwg::simple_message("Hello", &format!("Hello {}", self.ctl_amount.text()));
    }
    
    fn say_goodbye(&self) {
        nwg::simple_message("Goodbye", &format!("Goodbye {}", self.ctl_amount.text()));
        nwg::stop_thread_dispatch();
    }

}