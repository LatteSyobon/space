use nxui::io::storage::Storage;
use nxui::messagebox::MessageBox;
use nxui::natives_and_messaging::*;
use nxui::window::{Application, Frame, WindowAttributes};

pub struct TemplateApplication {

}

impl TemplateApplication {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl Application for TemplateApplication {
    fn app_name(&self) -> String {
        "NXUI Template App".to_string()
    }

    fn attributes(&self) -> WindowAttributes {
        WindowAttributes::new(WINDOWSTYLE_NORMAL,"Hello NXUI!".to_string(),1280,750,70,70,false)
    }

    fn startup(&self, _storage: Storage) {
        println!("Startup!")
    }

    fn ui(&self, frame: Frame) {
        frame.show();
        match MessageBox::new("Do you want to maximize the window?".to_string(), "Do you want to maximize the window?".to_string(), DIALOGSTYLE_QUESTION, BUTTONSTYLE_YESNO).show(frame.clone()) {
            RESULTS_YES => {
                frame.show_maximized();
                frame.set_title("The window has been maximized.".to_string());
            }

            RESULTS_NO => {
                frame.show_minimized();
                frame.set_title("The window has been minimized.".to_string());
            }

            _ => {}
        }
    }

    fn exit(&self) {
        println!("Exit!")
    }
}