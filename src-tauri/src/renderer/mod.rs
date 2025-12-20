// M3: Renderer Module
// Three-phase rendering pipeline for RPG prompts

pub mod seeded_random;
pub mod template_parser;
pub mod selector;
pub mod engine;
pub mod tag_expression; // M5 Phase 2: Complex tag expressions
pub mod separator;      // M5 Phase 3+4: Separator sets

// Export what's used by external modules (commands, CLI)
pub use engine::{Renderer, RenderResult};
