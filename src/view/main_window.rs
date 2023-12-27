use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::CompositeTemplate;
use gtk::{gio, glib};

use crate::app::UpdateApp;
use gtk::glib::subclass::InitializingObject;

use crate::{
    package_manager::{package_manager, set_package_manager},
    utils::IPackageManager,
    view::{DownloadPage, ErrorPage, LoadingPage, UpToDatePage, UpdatePage},
};

use super::done_page::DonePage;

const LOADING_PAGE_NAME: &str = "loading";
const UPDATE_PAGE_NAME: &str = "update";
const UP_TO_DATE_PAGE_NAME: &str = "up_to_date";
const DOWNLOAD_PAGE_NAME: &str = "download";
const ERROR_PAGE_NAME: &str = "error";
const DONE_PAGE_NAME: &str = "done";

mod imp {

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(file = "src/view/main_window.blp")]
    pub struct MainWindow {
        #[template_child]
        pub view_stack: TemplateChild<adw::ViewStack>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MainWindow {
        const NAME: &'static str = "MainWindow";
        type Type = super::MainWindow;
        type ParentType = adw::ApplicationWindow;

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
            let app = self.obj().clone();
            app.show_loading_page();

            glib::spawn_future_local(async move {
                if let Ok(packman) = IPackageManager::init().await {
                    set_package_manager(packman);
                    let packman = package_manager();
                    if packman.has_dist_upgrade() || packman.has_update() {
                        app.show_update_page();
                    } else {
                        app.show_up_to_date_page();
                    }
                } else {
                    app.show_error_page("Failed to initialize package manager");
                }
            });
        }
    }
    impl WidgetImpl for MainWindow {}
    impl WindowImpl for MainWindow {}
    impl AdwWindowImpl for MainWindow {}
    impl ApplicationWindowImpl for MainWindow {}
    impl AdwApplicationWindowImpl for MainWindow {}
}

glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<imp::MainWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
                 adw::ApplicationWindow,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
                    gtk::Native, gtk::Root, gtk::ShortcutManager,
                    gio::ActionGroup, gio::ActionMap;
}

impl MainWindow {
    pub fn new(app: &impl IsA<adw::Application>) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    pub fn show_loading_page(&self) {
        self.set_deletable(true);
        self.imp()
            .view_stack
            .add_titled(&LoadingPage::new(), Some(LOADING_PAGE_NAME), "Loading");
        self.imp()
            .view_stack
            .set_visible_child_name(LOADING_PAGE_NAME);
    }

    pub fn show_update_page(&self) {
        self.set_deletable(true);
        self.imp().view_stack.add_titled(
            &UpdatePage::new(),
            Some(UPDATE_PAGE_NAME),
            "Updates Available",
        );
        self.imp()
            .view_stack
            .set_visible_child_name(UPDATE_PAGE_NAME);
    }

    pub fn show_up_to_date_page(&self) {
        self.set_deletable(true);
        self.imp().view_stack.add_titled(
            &UpToDatePage::new(),
            Some(UP_TO_DATE_PAGE_NAME),
            "Up To Date",
        );
        self.imp()
            .view_stack
            .set_visible_child_name(UP_TO_DATE_PAGE_NAME);
    }

    pub fn show_download_page(&self) {
        self.set_deletable(false);
        self.imp().view_stack.add_titled(
            &DownloadPage::new(),
            Some(DOWNLOAD_PAGE_NAME),
            "Downloading",
        );
        self.imp()
            .view_stack
            .set_visible_child_name(DOWNLOAD_PAGE_NAME);
    }

    pub fn show_error_page(&self, message: &str) {
        self.set_deletable(true);
        self.imp()
            .view_stack
            .add_titled(&ErrorPage::new(message), Some(ERROR_PAGE_NAME), "Error");
        self.imp()
            .view_stack
            .set_visible_child_name(ERROR_PAGE_NAME);
    }

    pub fn show_done_page(&self) {
        self.set_deletable(true);
        self.imp()
            .view_stack
            .add_titled(&DonePage::new(), Some(DONE_PAGE_NAME), "Done");
        self.imp().view_stack.set_visible_child_name(DONE_PAGE_NAME);
    }
}

impl Default for MainWindow {
    fn default() -> Self {
        UpdateApp::default().main_window().clone()
    }
}
