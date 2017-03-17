use super::gtk;
use super::gtk::*;

use gui::page::Page;
use gui::page::Helper;

/// Name of the page.
const PAGE_NAME: &'static str = "Volume";

/// Volume page.
pub struct Volume {
    /// Page container
    container: gtk::Grid
}

impl Volume {

    /// Constructor.
    pub fn new() -> Self {
        // Create the page instance
        let page = Volume {
            container: Helper::create_page_container()
        };

        // Build the ui
        page.build_page();

        page
    }
}

impl Page for Volume {

    fn page_name(&self) -> &'static str {
        &PAGE_NAME
    }

    fn build_page(&self) {
        // Add a volume slider
        let slider = gtk::Scale::new_with_range(gtk::Orientation::Vertical, 0f64, 100f64, 0.1f64);
        slider.add_mark(50f64, PositionType::Right, "M");
        slider.set_vexpand(true);
        self.container.add(&slider);
    }

    fn gtk_widget(&self) -> &gtk::Grid {
        &self.container
    }
}