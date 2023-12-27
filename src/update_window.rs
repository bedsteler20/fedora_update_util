use std::time::Duration;

use crate::utils::load_img;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::gio;
use gtk::glib;
use gtk::glib::subclass::InitializingObject;
use gtk::CompositeTemplate;
use std::cell::RefCell;
use vte::prelude::*;

mod imp {

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(file = "src/update_window.blp")]
    pub struct UpdateWindow {
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
    impl ObjectSubclass for UpdateWindow {
        const NAME: &'static str = "UpdateWindow";
        type Type = super::UpdateWindow;
        type ParentType = adw::PreferencesWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for UpdateWindow {
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
                .set_paintable(Some(&load_img(include_bytes!("updates-available.svg"))));
            self.output_win.set_transient_for(Some(&self.obj().clone()));
            self.output_win.set_modal(true);
        }
    }

    #[gtk::template_callbacks]
    impl UpdateWindow {
        #[template_callback]
        fn on_show_output_btn(&self, _button: &gtk::Button) {
            self.output_win.present();
        }

        #[template_callback]
        fn on_term_child_exited(&self, status: i32, _term: &vte::Terminal) {
            let app = self
                .obj()
                .application()
                .unwrap()
                .downcast::<crate::UpdateApp>()
                .unwrap();

            if status == 0 {
                app.show_restart_win();
            } else {
                let dialog = adw::MessageDialog::builder()
                    .transient_for(&self.obj().clone())
                    .heading("Error")
                    .body("Error")
                    .build();
                dialog.add_response("ok", "Ok");
                dialog.connect_response(Some("ok"), move |_, _| app.quit());
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

    impl WidgetImpl for UpdateWindow {}
    impl WindowImpl for UpdateWindow {}
    impl AdwWindowImpl for UpdateWindow {}
    impl PreferencesWindowImpl for UpdateWindow {}
}

glib::wrapper! {
    pub struct UpdateWindow(ObjectSubclass<imp::UpdateWindow>)
    @extends adw::Window, gtk::Window, gtk::Widget,
             adw::PreferencesWindow,
    @implements gtk::Accessible, gtk::Buildable,
                gtk::ConstraintTarget, gtk::Native,
                gtk::Root, gtk::ShortcutManager;
}

impl UpdateWindow {
    pub fn new(app: &impl IsA<adw::Application>) -> Self {
        glib::Object::builder().property("application", app).build()
    }
}
