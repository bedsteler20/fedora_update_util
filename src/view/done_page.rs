use adw::subclass::prelude::*;
use gtk::glib;
use gtk::CompositeTemplate;

mod imp {
    use gtk::glib::subclass::InitializingObject;

    use crate::{package_manager::package_manager, utils::load_img};

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(file = "src/view/done_page.blp")]
    pub struct DonePage {
        #[template_child]
        pub status_page: TemplateChild<adw::StatusPage>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DonePage {
        const NAME: &'static str = "DonePage";
        type Type = super::DonePage;
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
    impl DonePage {
        #[template_callback]
        pub fn restart(_btn: &gtk::Button) {
            package_manager().reboot();
        }
    }

    impl ObjectImpl for DonePage {
        fn constructed(&self) {
            self.parent_constructed();

            self.status_page
                .set_paintable(Some(&load_img(include_bytes!("../up-to-date.svg"))));
        }
    }
    impl WidgetImpl for DonePage {}
    impl BinImpl for DonePage {}
}

glib::wrapper! {
    pub struct DonePage(ObjectSubclass<imp::DonePage>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl DonePage {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
