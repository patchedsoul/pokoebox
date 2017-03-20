#[cfg(feature = "rpi")]
extern crate cupi;

pub mod sig;
#[cfg(feature = "rpi")]
pub mod sig_gpio;
pub mod sig_in;
#[cfg(feature = "rpi")]
pub mod sig_in_gpio;
pub mod sig_in_toggle;
pub mod sig_out;
#[cfg(feature = "rpi")]
pub mod sig_out_gpio;
#[cfg(feature = "rpi")]
pub mod sig_out_gpio_light;
pub mod sig_out_light;
