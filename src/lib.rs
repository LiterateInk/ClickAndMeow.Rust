#[cfg(feature = "ffi")]
uniffi::setup_scaffolding!();

mod session;
pub use session::Session;

mod definitions;
pub use definitions::{Dishes, Menu};

mod error;
pub use error::Error;