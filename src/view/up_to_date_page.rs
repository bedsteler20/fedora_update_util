use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::CompositeTemplate;
use gtk::{gio, glib};

mod imp {
    use gtk::glib::subclass::InitializingObject;

    use crate::utils::load_img;

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(file = "src/view/up_to_date_page.blp")]
    pub struct UpToDatePage {
        #[template_child]
        pub status_page: TemplateChild<adw::StatusPage>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for UpToDatePage {
        const NAME: &'static str = "UpToDatePage";
        type Type = super::UpToDatePage;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for UpToDatePage {
        fn constructed(&self) {
            self.parent_constructed();
            self.status_page
                .set_paintable(Some(&load_img(include_bytes!("../up-to-date.svg"))));
        }
    }
    impl WidgetImpl for UpToDatePage {}
    impl BinImpl for UpToDatePage {}
}

glib::wrapper! {
    pub struct UpToDatePage(ObjectSubclass<imp::UpToDatePage>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl UpToDatePage {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
