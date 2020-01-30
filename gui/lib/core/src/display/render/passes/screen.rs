//! Pass rendering directly to the screen.

use crate::prelude::*;

use crate::display::render::pipeline::*;
use crate::display::symbol::Screen;
use crate::display::world::World;
use crate::system::gpu::*;



// ========================
// === ScreenRenderPass ===
// ========================

/// Renders the last `'color'` variable to the screen.
#[derive(Clone,Debug)]
pub struct ScreenRenderPass {
    screen: Screen,
}

impl ScreenRenderPass {
    /// Constructor.
    pub fn new(world:&World) -> Self {
        let screen = Screen::new(world);
        Self {screen}
    }
}

impl RenderPass for ScreenRenderPass {
    fn run(&mut self, _:&Context, _:&UniformScope) {
        self.screen.render();
    }
}