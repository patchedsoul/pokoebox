use super::gtk;
use super::gtk::*;

/// Main UI header in the application.
pub struct Header {
    container: gtk::Box
}

impl Header {

    /// Construct a new header.
    pub fn new() -> Self {
        Header {
            container: Self::build_container()
        }
    }

    /// Build the header container.
    fn build_container() -> gtk::Box {
        // Create a container instance
        let container = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        // Configure the header
        container.set_hexpand(true);
        container.set_vexpand(false);
        container.set_halign(gtk::Align::Fill);
        container.set_border_width(8);

        // Build the container controls
        Self::build_container_controls(&container);

        container
    }

    /// Build and add container controls to the container
    fn build_container_controls(container: &gtk::Box) {
        // Create a left button box
        let buttons_left = gtk::ButtonBox::new(gtk::Orientation::Horizontal);
        container.pack_start(&buttons_left, false, false, 0);

        // Create a home button
        let home_button = gtk::Button::new_with_label("Home");
        buttons_left.add(&home_button);
    }

    /// Get the GTK widget for this header.
    pub fn gtk_widget(&self) -> &gtk::Box {
        &self.container
    }
}