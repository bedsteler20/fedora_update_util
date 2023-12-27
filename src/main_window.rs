use crate::utils::load_img;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;
use gtk::glib::subclass::InitializingObject;
use gtk::CompositeTemplate;

mod imp {

    pub use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(file = "src/main_window.blp")]
    pub struct MainWindow {
        #[template_child]
        pub update_group: TemplateChild<adw::PreferencesGroup>,
        #[template_child]
        pub up_to_date_group: TemplateChild<adw::PreferencesGroup>,
        #[template_child]
        pub update_status: TemplateChild<adw::StatusPage>,
        #[template_child]
        pub up_to_date_status: TemplateChild<adw::StatusPage>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MainWindow {
        const NAME: &'static str = "MainWindow";
        type Type = super::MainWindow;
        type ParentType = adw::PreferencesWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MainWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let packman = crate::package_manager::package_manager();
            if packman.has_dist_upgrade() || packman.has_update() {
                self.update_group.set_visible(true);
                self.update_status
                    .set_paintable(Some(&load_img(include_bytes!("updates-available.svg"))));
                if packman.has_dist_upgrade() {
                    self.update_status.set_title(&packman.dist_update_msg());
                    self.update_status
                        .set_description(Some(&packman.dist_update_desertion()));
                } else {
                    self.update_status.set_title("Updates available");
                    self.update_status
                        .set_description(Some("Click to update your system"));
                }
            } else {
                self.up_to_date_group.set_visible(true);
                self.up_to_date_status
                    .set_paintable(Some(&load_img(include_bytes!("up-to-date.svg"))));
            }
        }
    }

    impl WidgetImpl for MainWindow {}
    impl WindowImpl for MainWindow {}
    impl AdwWindowImpl for MainWindow {}
    impl PreferencesWindowImpl for MainWindow {}
}

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends adw::Window, gtk::Window, gtk::Widget,
                 adw::PreferencesWindow,
        @implements gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native,
                    gtk::Root, gtk::ShortcutManager;
}

impl MainWindow {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
