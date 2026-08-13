use vello::{
    Scene,
    kurbo::{Affine, Point, Rect, Size},
};

use crate::el::El;

#[derive(Debug, Clone, Copy)]
pub struct CameraState {
    pub position: Point,
    pub zoom: f64,
    pub rotation: f64,
    pub viewport: Size,
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            position: Point::ORIGIN,
            zoom: 1.0,
            rotation: 0.0,
            viewport: Size::ZERO,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    transform: Affine,
    state: CameraState,
    dirty: bool,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            transform: Affine::IDENTITY,
            state: CameraState::default(),
            dirty: true,
        }
    }
}

impl Camera {
    pub fn new(viewport: Size) -> Self {
        let mut camera = Self::default();
        camera.state.viewport = viewport;
        camera
    }

    #[inline(always)]
    pub fn state_mut(&mut self) -> &mut CameraState {
        self.dirty = true;
        &mut self.state
    }

    #[inline(always)]
    pub fn transform(&mut self) -> Affine {
        if self.dirty {
            self.transform = Affine::translate((self.state.viewport * 0.5).to_vec2())
                * Affine::rotate(self.state.rotation)
                * Affine::scale(self.state.zoom)
                * Affine::translate(-self.state.position.to_vec2());
            self.dirty = false;
        }
        self.transform
    }

    /// Axis-aligned world-space rect covering the viewport.
    /// O(1): no matrix build, no inverse, no per-corner transform.
    #[inline(always)]
    pub fn visible_world_rect(&self) -> Rect {
        let s = &self.state;
        let inv_zoom = 1.0 / s.zoom.abs();
        let hw = s.viewport.width * 0.5 * inv_zoom;
        let hh = s.viewport.height * 0.5 * inv_zoom;

        let (half_x, half_y) = if s.rotation == 0.0 {
            (hw, hh)
        } else {
            let (sin_t, cos_t) = s.rotation.sin_cos();
            let (sin_t, cos_t) = (sin_t.abs(), cos_t.abs());
            // FMA-friendly form: cos_t*hw + sin_t*hh
            (cos_t.mul_add(hw, sin_t * hh), sin_t.mul_add(hw, cos_t * hh))
        };

        Rect::new(
            s.position.x - half_x,
            s.position.y - half_y,
            s.position.x + half_x,
            s.position.y + half_y,
        )
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        *self = Self::new(self.state.viewport);
    }

    pub fn render<'a, I>(&mut self, scene: &mut Scene, els: I)
    where
        I: IntoIterator<Item = &'a El>,
    {
        let camera_transform = self.transform();
        let visible = self.visible_world_rect();

        for el in els {
            let bbox = el.world_bounding_box();
            let r = visible.intersect(bbox);
            if r.width() > 0.0 && r.height() > 0.0 {
                el.render_with_base(scene, camera_transform);
            }
        }
    }
}
