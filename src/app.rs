use gtk::gio;
use gtk::glib;

use crate::package_manager::PackageManager;
use crate::utils;
use crate::utils::IPackageManager;
use crate::utils::APP_ID;
use crate::view::{DownloadPage, ErrorPage, LoadingPage, MainWindow, UpdatePage};
use adw::prelude::*;
use adw::subclass::prelude::*;
use gio::prelude::*;
use gtk::glib::once_cell::sync::OnceCell;
use gtk::glib::subclass::InitializingType;
mod imp {

    pub use super::*;

    #[derive(Debug)]
    pub struct UpdateApp {
        pub main_window: OnceCell<MainWindow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for UpdateApp {
        const NAME: &'static str = "UpdateApp";
        type Type = super::UpdateApp;
        type ParentType = adw::Application;

        fn new() -> Self {
            Self {
                main_window: OnceCell::new(),
            }
        }

        fn type_init(_klass: &mut InitializingType<Self>) {
            DownloadPage::ensure_type();
            UpdatePage::ensure_type();
            ErrorPage::ensure_type();
            LoadingPage::ensure_type();
        }
    }

    impl ObjectImpl for UpdateApp {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl ApplicationImpl for UpdateApp {
        fn activate(&self) {
            println!("UpdateApp::activate");
            self.obj().main_window().present();
        }

        fn command_line(&self, command_line: &gio::ApplicationCommandLine) -> glib::ExitCode {
            if command_line.arguments().contains(&"--check-updates".into()) {
                self.obj().check_updates();
                glib::ExitCode::SUCCESS
            } else {
                self.activate();
                glib::ExitCode::SUCCESS
            }
        }
    }
    impl GtkApplicationImpl for UpdateApp {}
    impl AdwApplicationImpl for UpdateApp {}
}

glib::wrapper! {
    pub struct UpdateApp(ObjectSubclass<imp::UpdateApp>)
        @extends gtk::Application, gio::Application, adw::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl UpdateApp {
    pub fn new() -> Self {
        let app: Self = glib::Object::builder().build();
        app.set_flags(gio::ApplicationFlags::HANDLES_COMMAND_LINE);
        app.set_application_id(Some(utils::APP_ID));
        app
    }

    pub fn main_window(&self) -> &MainWindow {
        if let Some(main_win) = self.imp().main_window.get() {
            return main_win;
        } else {
            self.imp().main_window.set(MainWindow::new(self)).unwrap();
        }

        self.imp().main_window.get().unwrap()
    }

    pub fn check_updates(&self) {
        println!("Checking for updates");
        let app = glib::MainContext::default().block_on(async { IPackageManager::init().await });
        if let Err(err) = app {
            panic!("Failed to initialize package manager: {}", err);
        }
        let packman = app.unwrap();
        if true || packman.has_dist_upgrade() || packman.has_update() {
            println!("Updates available");
            let notification = gio::Notification::new("Updates available");
            notification.set_body(Some("Updates are available for your system"));
            // notification.set_icon(&gio::Icon::for_string(APP_ID).unwrap());
            notification.set_priority(gio::NotificationPriority::Urgent);
            self.send_notification(Some("UwU"), &notification);
        } else {
            println!("No updates available");
        }
    }
}

impl Default for UpdateApp {
    fn default() -> Self {
        gio::Application::default().unwrap().downcast().unwrap()
    }
}
