use std::sync::Arc;

use glib::clone;
use gtk::prelude::*;

use crate::action::actions::GotoPageAction;
use crate::app::Core;
use crate::pages::PageType;

use super::page::Helper;
use super::page::Page;

const PAGE_TYPE: PageType = PageType::Launchpad;
const PAGE_NAME: &str = "Launchpad";
const BUTTON_SPACING: u32 = 16;
const BUTTON_GRID_SIZE: (i32, i32) = (450, 260);

/// Launchpad.
pub struct Launchpad {
    container: gtk::Grid,
}

impl Launchpad {
    /// Constructor.
    pub fn new(core: Arc<Core>) -> Self {
        // Create the page instance
        let page = Self {
            container: Helper::create_page_container(),
        };

        // Build the page ui
        page.build_page(core);

        page
    }
}

impl Page for Launchpad {
    fn page_type(&self) -> PageType {
        PAGE_TYPE
    }

    fn page_name(&self) -> &'static str {
        &PAGE_NAME
    }

    fn build_page(&self, core: Arc<Core>) {
        // Configure the page
        self.container.set_halign(gtk::Align::Center);
        self.container.set_valign(gtk::Align::Center);

        // Create a button grid
        let btns = gtk::Grid::new();
        btns.set_row_spacing(BUTTON_SPACING);
        btns.set_column_spacing(BUTTON_SPACING);
        btns.set_row_homogeneous(true);
        btns.set_column_homogeneous(true);
        btns.set_size_request(BUTTON_GRID_SIZE.0, BUTTON_GRID_SIZE.1);
        self.container.add(&btns);

        // Add some buttons
        let btn_play = gtk::Button::new_with_label("Play");
        btn_play.connect_clicked(clone!(@weak core => move |_| {
            core
                .actions
                .invoke(GotoPageAction::new(PageType::Player), core.clone());
        }));
        btns.attach(&btn_play, 0, 0, 1, 1);

        let btn_bluetooth = gtk::Button::new_with_label("Bluetooth");
        #[cfg(feature = "bluetooth")]
        btn_bluetooth.connect_clicked(clone!(@weak core => move |_| {
            core.actions.invoke(
                GotoPageAction::new(PageType::Bluetooth),
                core.clone(),
            );
        }));
        #[cfg(not(feature = "bluetooth"))]
        btn_bluetooth.set_sensitive(false);
        btns.attach(&btn_bluetooth, 1, 0, 1, 1);

        let btn_c = gtk::Button::new_with_label("");
        btn_c.set_sensitive(false);
        btns.attach(&btn_c, 2, 0, 1, 1);

        let btn_d = gtk::Button::new_with_label("");
        btn_d.set_sensitive(false);
        btns.attach(&btn_d, 0, 1, 1, 1);

        let btn_soundboard = gtk::Button::new_with_label("Soundboard");
        btn_soundboard.connect_clicked(clone!(@weak core => move |_| {
            core.actions.invoke(
                GotoPageAction::new(PageType::Soundboard),
                core.clone(),
            );
        }));
        btns.attach(&btn_soundboard, 1, 1, 1, 1);

        let btn_volume = gtk::Button::new_with_label("Volume");
        btn_volume.connect_clicked(clone!(@weak core => move |_| {
            core
                .actions
                .invoke(GotoPageAction::new(PageType::Volume), core.clone());
        }));
        btns.attach(&btn_volume, 2, 1, 1, 1);
    }

    fn gtk_widget(&self) -> &gtk::Grid {
        &self.container
    }
}
