mod imp;

use gio::prelude::ApplicationExtManual;
use glib::subclass::prelude::ObjectSubclassIsExt;
use gtk::{gio, glib};
use crate::constants;
use crate::launcher::Launcher;

glib::wrapper! {
    pub struct RGLApplication(ObjectSubclass<imp::RGLApplication>) @extends gio::Application, gtk::Application, @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for RGLApplication {
    fn default() -> Self {
        Self::new()
    }
}

impl RGLApplication {
    pub fn new() -> Self {
        glib::Object::builder()
            .property("application-id", constants::APP_ID)
            .build()
    }

    pub fn set_launcher(&mut self, launcher: Launcher) {
        self.imp().launcher.set(launcher).expect("TODO: panic message");
    }

    pub fn set_hold(&self) {
        self.imp().background_hold.set(self.hold()).expect("Unable set background hold");
    }
}