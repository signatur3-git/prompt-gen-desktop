// M4: Context Module
// Scoped key-value storage for coordinating between template elements

pub mod context;
pub mod value;

pub use context::{Context, ContextError};
pub use value::ContextValue;

