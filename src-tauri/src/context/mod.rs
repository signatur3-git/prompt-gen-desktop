// M4: Context Module
// Scoped key-value storage for coordinating between template elements

pub mod storage;
pub mod value;

pub use storage::{Context, ContextError};
pub use value::ContextValue;
