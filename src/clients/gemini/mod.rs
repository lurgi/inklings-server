mod client;
mod mock;
mod traits;

#[cfg(test)]
mod tests;

pub use client::GeminiClient;
pub use mock::MockGeminiClient;
pub use traits::{Embedder, TextGenerator};
