// Module declarations
pub mod ai;
pub mod camera;
pub mod game;
pub mod game_over;
pub mod menu;
pub mod rendering;
pub mod scoreboard;

// Re-export all public functions for backward compatibility
pub use ai::*;
pub use camera::*;
pub use game::*;
pub use game_over::*;
pub use menu::*;
pub use rendering::*;
pub use scoreboard::*;
