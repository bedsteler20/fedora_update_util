mod dnf;
mod main_window;
mod update_window;
mod utils;
mod app;

mod package_manager;
use adw::prelude::*;
use app::UpdateApp;
use gtk::glib;

fn main() -> glib::ExitCode {
    let app = UpdateApp::new();
    app.run()
}
