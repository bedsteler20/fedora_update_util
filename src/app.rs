use gtk::gio;
use gtk::glib;

use crate::utils;
use crate::view::{DownloadPage, ErrorPage, LoadingPage, MainWindow, UpdatePage};
use adw::prelude::*;
use adw::subclass::prelude::*;
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

    impl ObjectImpl for UpdateApp {}

    impl ApplicationImpl for UpdateApp {
        fn activate(&self) {
            println!("UpdateApp::activate");
            self.obj().main_window().present();
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
        glib::Object::builder()
            .property("application-id", utils::APP_ID)
            .build()
    }

    pub fn main_window(&self) -> &MainWindow {
        if let Some(main_win) = self.imp().main_window.get() {
            return main_win;
        } else {
            self.imp().main_window.set(MainWindow::new(self)).unwrap();
        }

        self.imp().main_window.get().unwrap()
    }
}

impl Default for UpdateApp {
    fn default() -> Self {
        gio::Application::default().unwrap().downcast().unwrap()
    }
}
