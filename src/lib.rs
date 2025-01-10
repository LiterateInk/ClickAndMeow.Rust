#[cfg(feature = "ffi")]
uniffi::setup_scaffolding!();

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

mod session;
pub use session::Session;

mod definitions;
pub use definitions::{Dishes, Menu, Date, Establishment};

mod error;
pub use error::Error;

mod menu;
pub use menu::{get_menus, get_menu_dishes};

mod establishment;
pub use establishment::get_establishments;

const BASE_URL: &str = "https://www.clicetmiam.fr";