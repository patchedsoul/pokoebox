use super::super::signal::traits::sig_out::SigOut;
use super::with_io::WithIo;

/// Defines that the peripheral has output signals.
/// This may be combined with `WithInputs`.
pub trait WithOutputs: WithIo {

    /// Get a vector of inputs this peripheral provides.
    fn outputs(&self) -> &Vec<Box<SigOut>>;
}