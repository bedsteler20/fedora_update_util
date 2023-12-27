use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::CompositeTemplate;
use gtk::{gio, glib};

mod imp {
    use gtk::glib::subclass::InitializingObject;

    use crate::{utils::load_img, app::UpdateApp, view::MainWindow};

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(file = "src/view/update_page.blp")]
    pub struct UpdatePage {
        #[template_child]
        pub status_page: TemplateChild<adw::StatusPage>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for UpdatePage {
        const NAME: &'static str = "UpdatePage";
        type Type = super::UpdatePage;
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
    impl UpdatePage {
        #[template_callback]
        fn on_btn_clicked(_btn: &gtk::Button) {
            MainWindow::default().show_download_page();
        }
    }


    
    impl ObjectImpl for UpdatePage {
        fn constructed(&self) {
            self.parent_constructed();
            self.status_page
                .set_paintable(Some(&load_img(include_bytes!("../updates-available.svg"))));

            let packman = crate::package_manager::package_manager();

            if packman.has_dist_upgrade() {
                self.status_page.set_title(&packman.dist_update_msg());
                self.status_page
                    .set_description(Some(&packman.dist_update_description()));
            }
        }
    }
    impl WidgetImpl for UpdatePage {}
    impl BinImpl for UpdatePage {}
}

glib::wrapper! {
    pub struct UpdatePage(ObjectSubclass<imp::UpdatePage>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl UpdatePage {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
