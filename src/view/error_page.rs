use gtk::CompositeTemplate;
use gtk::{gio, glib};
use gtk::prelude::*;
use adw::subclass::prelude::*;

mod imp {
    use gtk::glib::subclass::InitializingObject;

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(file = "src/view/error_page.blp")]
    pub struct ErrorPage {
        #[template_child]
        pub status_page: TemplateChild<adw::StatusPage>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ErrorPage {
        const NAME: &'static str = "ErrorPage";
        type Type = super::ErrorPage;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ErrorPage {}
    impl WidgetImpl for ErrorPage {}
    impl BinImpl for ErrorPage {}
}

glib::wrapper! {
    pub struct ErrorPage(ObjectSubclass<imp::ErrorPage>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl ErrorPage {
    pub fn new(message: &str) -> Self {
        let this: Self =  glib::Object::builder().build();
        println!("ErrorPage::new({})", message);
        this.imp().status_page.set_description(Some(&message));
        this
    }

}