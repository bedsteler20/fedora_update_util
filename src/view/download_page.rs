use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::CompositeTemplate;
use gtk::{gio, glib};
use vte::prelude::*;

mod imp {
    use std::{cell::RefCell, time::Duration};

    use gtk::glib::subclass::InitializingObject;

    use crate::{app::UpdateApp, view::MainWindow, utils::load_img};

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(file = "src/view/download_page.blp")]
    pub struct DownloadPage {
        #[template_child]
        pub status_page: TemplateChild<adw::StatusPage>,
        #[template_child]
        pub bar: TemplateChild<gtk::ProgressBar>,
        #[template_child]
        pub output_win: TemplateChild<adw::Window>,
        #[template_child]
        pub term: TemplateChild<vte::Terminal>,
        pub cancel: RefCell<gio::Cancellable>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DownloadPage {
        const NAME: &'static str = "DownloadPage";
        type Type = super::DownloadPage;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[gtk::template_callbacks]
    impl DownloadPage {
        #[template_callback]
        fn on_show_output_btn(&self, _button: &gtk::Button) {
            self.output_win.present();
        }

        #[template_callback]
        fn on_term_child_exited(&self, status: i32, _term: &vte::Terminal) {
            let win = MainWindow::default();
            self.output_win.close();

            if status == 0 {
                win.show_done_page();
            } else {
                win.show_error_page("Failed to update");
            }
        }

        fn spawn_command(&self, command: &[&str]) {
            let cancel = self.cancel.borrow().clone();
            self.term.spawn_async(
                vte::PtyFlags::DEFAULT,
                None,
                command,
                &[],
                gtk::glib::SpawnFlags::DEFAULT,
                || {},
                -1,
                Some(&cancel),
                move |_| {},
            )
        }
    }

    impl ObjectImpl for DownloadPage {
        fn constructed(&self) {
            let packman = crate::package_manager::package_manager();

            if packman.has_dist_upgrade() {
                self.spawn_command(&packman.get_dist_update_command());
            } else {
                self.spawn_command(&packman.get_update_command());
            }

            self.bar.pulse();

            glib::timeout_add_local(Duration::from_millis(500), {
                let bar = self.bar.clone();
                move || {
                    bar.pulse();
                    glib::ControlFlow::Continue
                }
            });

            self.status_page
                .set_paintable(Some(&load_img(include_bytes!("../updates-available.svg"))));
            self.output_win.set_transient_for(Some(&MainWindow::default()));
            self.output_win.set_modal(true);
        }
    }
    impl WidgetImpl for DownloadPage {}
    impl BinImpl for DownloadPage {}
}

glib::wrapper! {
    pub struct DownloadPage(ObjectSubclass<imp::DownloadPage>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl DownloadPage {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
