use gtk::gio;
use gtk::glib;

use adw::subclass::prelude::*;

use adw::prelude::*;

use crate::main_window::MainWindow;
use crate::package_manager::package_manager;
use crate::package_manager::set_package_manager;
use crate::update_window::UpdateWindow;
use crate::utils;
use crate::utils::IPackageManager;

mod imp {
    pub use super::*;

    #[derive(Debug, Default)]
    pub struct UpdateApp;

    #[glib::object_subclass]
    impl ObjectSubclass for UpdateApp {
        const NAME: &'static str = "UpdateApp";
        type Type = super::UpdateApp;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for UpdateApp {
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().setup_actions();
        }
    }

    impl ApplicationImpl for UpdateApp {
        fn activate(&self) {
            self.obj().init_package_manager();
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

    fn show_loading_win(&self) {
        let content = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .margin_bottom(30)
            .margin_top(30)
            .margin_start(30)
            .margin_end(30)
            .build();

        let spinner = gtk::Spinner::builder().vexpand(true).hexpand(true).build();
        spinner.start();

        let label = gtk::Label::builder()
            .label("Checking for Updates")
            .margin_top(20)
            .css_classes(vec!["title-3"])
            .halign(gtk::Align::Center)
            .build();

        content.append(&spinner);
        content.append(&label);

        let window = adw::Window::builder()
            .title("Loading")
            .default_width(200)
            .default_height(200)
            .resizable(false)
            .application(self)
            .content(&content)
            .build();

        window.present();
    }

    fn init_package_manager(&self) {
        self.show_loading_win();

        let this = self.clone();

        glib::spawn_future_local(async move {
            let packman = IPackageManager::init().await;

            match packman {
                Err(e) => {
                    print!("error: {}", e);
                    let dialog = adw::MessageDialog::builder()
                        .body(&format!("Error: {}", e))
                        .title("Error")
                        .heading("Error")
                        .application(&this)
                        .build();
                    dialog.add_response("close", "Close");
                    dialog.connect_response(Some("close"), {
                        let app = this.clone();
                        move |_, _| app.quit()
                    });
                    dialog.present();
                }
                Ok(packman) => {
                    set_package_manager(packman);
                    print!("has update: {}", package_manager().has_update());
                    this.show_main_win();
                }
            }
        });
    }

    fn close_all_windows(&self) {
        for win in self.windows() {
            win.close();
        }
    }

    fn show_main_win(&self) {
        self.close_all_windows();
        let window = MainWindow::new();
        window.set_application(Some(self));
        window.present();
    }

    fn show_update_win(&self) {
        self.close_all_windows();
        let window = UpdateWindow::new(self);
        window.present();
    }

    pub fn show_restart_win(&self) {
        self.close_all_windows();
        let dialog = adw::MessageDialog::builder()
            .body("Please restart your system.")
            .title("Updates Complete")
            .heading("Updates Complete")
            .build();
        dialog.add_response("later", "Restart Later");
        dialog.add_response("now", "Restart Now");
        dialog.connect_response(Some("later"), {
            let app = self.clone();
            move |_, _| app.quit()
        });
        dialog.connect_response(Some("now"), {
            let app = self.clone();
            move |_, _| {
                package_manager().reboot();
                app.quit();
            }
        });
        dialog.present();
    }

    fn setup_actions(&self) {
        self.add_action_entries([
            gio::ActionEntry::builder("show_main_win")
                .activate({
                    let app = self.clone();
                    move |_, _, _| app.show_main_win()
                })
                .build(),
            gio::ActionEntry::builder("restart")
                .activate({
                    let app = self.clone();
                    move |_, _, _| app.show_restart_win()
                })
                .build(),
            gio::ActionEntry::builder("show_update_win")
                .activate({
                    let app = self.clone();
                    move |_, _, _| app.show_update_win()
                })
                .build(),
        ]);
    }
}
