use std::boxed::Box;
use std::sync::Arc;

use crate::app::Core;
use crate::error::Error;
use crate::result::Result;

use super::main::App;
use super::pages::{launchpad::Launchpad, test::Test, volume::Volume};
use super::Window;

/// Main gui object, which manages the graphical interface side of the
/// application.
///
/// Creating a new instance will automatically initialize GTK.
pub struct Ui {
    /// Main window.
    window: Window,

    /// Main app UI.
    app_ui: App,
}

impl Ui {
    /// Constructor.
    ///
    /// Constructing the object will initialize the GTK toolkit.
    ///
    /// # Errors
    ///
    /// Returns an error if GTK failed to initialize,
    /// blocking further GTK usage.
    pub fn new(core: Arc<Core>) -> Result<Self> {
        // Initialize GTK
        debug!("Initializing GTK...");
        if gtk::init().is_err() {
            return Err(Error::new("Failed to initialize GTK"));
        }
        debug!("Successfully initialized GTK.");

        // Build UI
        let (window, app_ui) = Self::build_ui(core);
        let gui = Self { window, app_ui };

        // Show window
        info!("Showing master GUI frame...");
        gui.window.show();

        Ok(gui)
    }

    /// Set up the main gui interface.
    /// This creates a window or frame, and builds the interface in it.
    /// Nothing happens if a master frame is already available.
    fn build_ui(core: Arc<Core>) -> (Window, App) {
        // Create window and app UI
        let window = Window::new();
        let mut app = App::new(core.clone());

        // Put app UI in window
        window.set_app(&app);

        // Add the launchpad page
        let launchpad = Launchpad::new(core.clone());
        app.pages.add_page(Box::new(launchpad));

        // Add the test page
        let test = Test::new(core.clone());
        app.pages.add_page(Box::new(test));

        // Add the volume page
        let volume = Volume::new(core);
        app.pages.add_page(Box::new(volume));

        (window, app)
    }

    /// Run the main loop of the GUI.
    pub fn main(&self) {
        gtk::main();
    }
}
