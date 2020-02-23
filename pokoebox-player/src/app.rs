use std::sync::Arc;

use pokoebox_bluetooth::Manager as BluetoothManager;

use crate::action::ActionRuntime;
use crate::result::Result;
use crate::ui::gtk::Ui;

use super::pages::PageController;

pub struct App {
    ui: Ui,
    pub core: Arc<Core>,
}

impl App {
    pub fn new() -> Result<Self> {
        // Init app core
        let core = Arc::new(Core::new()?);

        Ok(Self {
            ui: Ui::new(core.clone())?,
            core,
        })
    }

    pub fn run(self) -> Self {
        // Run the GUIs main loop
        loop {
            self.ui.main();
        }
    }
}

pub struct Core {
    /// Action manager
    pub actions: ActionRuntime,

    /// Bluetooth manager.
    pub bluetooth: BluetoothManager,

    pub pages: PageController,
}

impl Core {
    pub fn new() -> Result<Self> {
        Ok(Self {
            actions: ActionRuntime::default(),
            // TODO: propagate error
            bluetooth: BluetoothManager::new().expect("failed to initialize bluetooth manager"),
            pages: PageController::new(),
        })
    }
}
