use adw::subclass::prelude::*;
use gtk::glib;

mod imp {
    use gtk::{glib::subclass::InitializingObject, CompositeTemplate};

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(file = "src/view/loading_page.blp")]
    pub struct LoadingPage {}

    #[glib::object_subclass]
    impl ObjectSubclass for LoadingPage {
        const NAME: &'static str = "LoadingPage";
        type Type = super::LoadingPage;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for LoadingPage {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl WidgetImpl for LoadingPage {}
    impl BinImpl for LoadingPage {}
}

glib::wrapper! {
    pub struct LoadingPage(ObjectSubclass<imp::LoadingPage>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl LoadingPage {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
