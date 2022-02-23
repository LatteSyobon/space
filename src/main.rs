use nxui_template::app;

fn main() {
    let window = app::TemplateApplication::new();
    nxui::nxui::create_new_app(Box::new(window));
}
