//! World Map Component
//!
//! WebAssembly component for rendering interactive world maps in the browser.

use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use wmc_core::{projection::MercatorProjection, theme::Theme, topology::WorldTopology};
use wmc_render::{RenderContext, WorldRenderer};

/// Component error types
pub mod error;

pub use error::ComponentError;

const WORLD_GEOJSON: &str = include_str!("../../../assets/world-110m.geojson");

/// World map component for WebAssembly
#[wasm_bindgen]
pub struct WorldMap {
    ctx: RenderContext,
    world_renderer: WorldRenderer,
    theme: Theme,
    topology: WorldTopology,
}

#[wasm_bindgen]
impl WorldMap {
    /// Creates a new world map component
    ///
    /// # Errors
    ///
    /// Returns `JsValue` error if WebGL initialization or topology parsing fails
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: &HtmlCanvasElement) -> Result<Self, JsValue> {
        console_error_panic_hook::set_once();

        let ctx = RenderContext::new(canvas)
            .map_err(|e| JsValue::from_str(&format!("WebGL init failed: {e}")))?;

        let topology = WorldTopology::from_geojson(WORLD_GEOJSON)
            .map_err(|e| JsValue::from_str(&format!("Topology parse failed: {e}")))?;

        let projection = MercatorProjection::new(f64::from(ctx.width()), f64::from(ctx.height()));

        let world_renderer = WorldRenderer::new(&ctx, &topology, &projection)
            .map_err(|e| JsValue::from_str(&format!("Renderer init failed: {e}")))?;

        let theme = Theme::dark_minimal();

        Ok(Self {
            ctx,
            world_renderer,
            theme,
            topology,
        })
    }

    /// Resizes the map viewport
    ///
    /// # Errors
    ///
    /// Returns `JsValue` error if renderer reinitialization fails
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), JsValue> {
        self.ctx.resize(width, height);

        let projection = MercatorProjection::new(f64::from(width), f64::from(height));

        self.world_renderer = WorldRenderer::new(&self.ctx, &self.topology, &projection)
            .map_err(|e| JsValue::from_str(&format!("Renderer reinit failed: {e}")))?;

        Ok(())
    }

    /// Renders the map to the canvas
    pub fn render(&self) {
        self.ctx.clear(
            self.theme.background.r,
            self.theme.background.g,
            self.theme.background.b,
            self.theme.background.a,
        );

        self.world_renderer.draw(
            &self.ctx,
            [
                self.theme.contour_color.r,
                self.theme.contour_color.g,
                self.theme.contour_color.b,
                self.theme.contour_color.a,
            ],
            self.theme.contour_width,
        );
    }
}
