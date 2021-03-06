#![cfg(feature = "old-rpi")]

use std::collections::HashMap;

use super::sig_id::SigId;
use super::traits::sig::Sig;
use super::traits::sig_gpio::SigGpio;
use super::traits::sig_in::SigIn;
use super::traits::sig_in_gpio::SigInGpio;
use super::traits::sig_in_gpio_toggle::SigInGpioToggle;
use super::traits::sig_in_toggle::SigInToggle;
use error::Error;
use gpio::gpio_manager::GpioManager;
use gpio::pin::Pin;
use gpio::pin_accessor::PinAccessor;
use gpio::pin_config::{IoMode, PinConfig, PullMode};
use gpio::pin_token::PinToken;
use result::Result;

/// Key for the GPIO pin of the light
pub const GPIO_PIN_KEY_BUTTON: &'static str = "light";

/// Output signal for a peripheral to control a light.
pub struct InputGpioToggle {
    id: SigId,
    name: &'static str,
    pin_configs: HashMap<&'static str, PinConfig>,
    pins: HashMap<&'static str, PinToken>,
}

impl InputGpioToggle {
    /// Create a new instance.
    /// The GPIO pin of the button must be passed to the `pin` parameter.
    pub fn new(
        id: SigId,
        name: &'static str,
        pin: usize,
        gpio_manager: &mut GpioManager,
    ) -> Result<Self> {
        // Create a hash map of pin configurations
        let mut pin_configs = HashMap::new();

        // Create a pin configuration
        let mut pin_config = PinConfig::new_with_pin_and_io(pin, IoMode::Input);
        pin_config.set_pull_mode(PullMode::PullUp);
        pin_config.set_inverted(true);

        // Create the pin configuration, and add it to the configurations list
        pin_configs.insert(GPIO_PIN_KEY_BUTTON, pin_config);

        // Construct the object
        let mut obj = InputGpioToggle {
            id,
            name,
            pin_configs,
            pins: HashMap::new(),
        };

        // Setup the pins
        obj.setup_pins(gpio_manager)?;

        Ok(obj)
    }

    /// Find the button signal pin.
    fn find_button_pin<'a, 'b: 'a>(&'a self, pin_accessor: &'b PinAccessor) -> Option<&'a Pin> {
        self.gpio_pin(GPIO_PIN_KEY_BUTTON, pin_accessor)
    }
}

impl Sig for InputGpioToggle {
    fn id(&self) -> &SigId {
        &self.id
    }

    fn name(&self) -> &'static str {
        &self.name
    }
}

impl SigGpio for InputGpioToggle {
    fn gpio_pin_configs(&self) -> &HashMap<&'static str, PinConfig> {
        &self.pin_configs
    }

    fn gpio_pin_configs_mut(&mut self) -> &mut HashMap<&'static str, PinConfig> {
        &mut self.pin_configs
    }

    fn gpio_pin_tokens(&self) -> &HashMap<&'static str, PinToken> {
        &self.pins
    }

    fn add_gpio_pin_token(&mut self, key: &'static str, pin_token: PinToken) {
        self.pins.insert(key, pin_token);
    }
}

impl SigIn for InputGpioToggle {}

impl SigInGpio for InputGpioToggle {}

impl SigInToggle for InputGpioToggle {}

impl SigInGpioToggle for InputGpioToggle {
    fn state(&self, pin_accessor: &PinAccessor) -> Result<bool> {
        self.find_button_pin(pin_accessor)
            .and_then(|pin| Some(pin.read_bool()))
            .ok_or(Error::new(
                "Failed to read button state, unable to find button pin.",
            ))
    }
}
