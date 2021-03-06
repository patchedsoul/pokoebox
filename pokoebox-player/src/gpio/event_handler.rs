#![cfg(feature = "old-rpi")]

use super::trigger_edge::TriggerEdge;
use result::Result;

/// Event handler trait, to handle GPIO events.
pub trait EventHandler {
    /// Invoke the event, unmutable.
    /// Returns whether the event was consumed or not.
    /// `true` is returned if the event is consumed/used, `false` if it isn't.
    /// When `false` is returned, another handler is chosen to handle the event.
    ///
    /// An error is returned if this event can't be invoked in an unmutable way.
    /// Therefore it's recommended to use `invoke_mut` instead, at all times.
    fn invoke(&self) -> Result<bool>;

    /// Invoke the event.
    /// Returns whether the event was consumed or not.
    /// `true` is returned if the event is consumed/used, `false` if it isn't.
    /// When `false` is returned, another handler is chosen to handle the event.
    fn invoke_mut(&mut self) -> Result<bool>;

    /// Get the trigger edge type for this event.
    /// This defines on what signalling edges this event should be fired.
    fn trigger_edge(&self) -> TriggerEdge;
}
