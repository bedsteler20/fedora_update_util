mod dnf;
mod utils;
mod app;
mod view;

mod package_manager;
use adw::prelude::*;
use app::UpdateApp;
use gtk::glib;

fn main() -> glib::ExitCode {
    let app = UpdateApp::new();
    app.set_default();
    app.run()
}
