use ::gtk4::prelude::*;

use ::gtk4::{Application, ApplicationWindow};

fn main() {
    let app = Application::builder()
        .application_id("com.demoapp.demoapp")
        .build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .title("Ui Demo")
        .application(app)
        .build();

    window.show();
}
