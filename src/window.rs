use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, CompositeTemplate};

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/org/gustavoperedo/FontDownloader/window.ui")]
    pub struct FontDownloaderWindow {
        // Template widgets
        #[template_child]
        pub header_bar: TemplateChild<gtk::HeaderBar>,
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for FontDownloaderWindow {
        const NAME: &'static str = "FontDownloaderWindow";
        type Type = super::FontDownloaderWindow;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for FontDownloaderWindow {}
    impl WidgetImpl for FontDownloaderWindow {}
    impl WindowImpl for FontDownloaderWindow {}
    impl ApplicationWindowImpl for FontDownloaderWindow {}
}

glib::wrapper! {
    pub struct FontDownloaderWindow(ObjectSubclass<imp::FontDownloaderWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl FontDownloaderWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::new(&[("application", application)])
            .expect("Failed to create FontDownloaderWindow")
    }
}
