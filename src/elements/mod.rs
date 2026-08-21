use vello::kurbo::{Affine, Point, Rect, Shape as _, Vec2};

mod any_shape;
mod style;

pub use any_shape::AnyShape;
pub use style::Style;

#[derive(Debug, Clone)]
pub enum ElementInner {
    Shape(AnyShape),
    Text(String),
    Group(Vec<Element>),
}

#[derive(Debug, Clone, Copy)]
pub struct ElementState {
    pub position: Point,
    pub rotation: f64,
    pub scale: Vec2,
    pub anchor: Vec2,
}

impl Default for ElementState {
    fn default() -> Self {
        Self {
            position: Point::ZERO,
            rotation: 0.0,
            scale: Vec2::new(1.0, 1.0),
            anchor: Vec2::new(0.5, 0.5),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Element {
    id: u64,

    inner: ElementInner,
    state: ElementState,

    local_bbox: Rect,
    world_bbox: Rect,

    transform: Affine,

    dirty: bool,
}

impl Element {
    #[inline(always)]
    pub fn inner(&self) -> &ElementInner {
        &self.inner
    }

    #[inline(always)]
    pub fn set_inner(&mut self, inner: ElementInner) {
        match inner {
            ElementInner::Shape(shape) => {
                self.local_bbox = shape.bounding_box();
            }
            ElementInner::Text(_) => todo!(),
            ElementInner::Group(_elements) => todo!(),
        }
        self.dirty = true;
    }

    #[inline(always)]
    pub fn state(&self) -> &ElementState {
        &self.state
    }

    #[inline(always)]
    pub fn state_mut(&mut self) -> &mut ElementState {
        self.dirty = true;
        &mut self.state
    }

    #[inline(always)]
    fn is_identity(&self) -> bool {
        self.state.position == Point::ZERO
            && self.state.rotation == 0.0
            && self.state.scale == Vec2::new(1.0, 1.0)
    }

    #[inline(always)]
    fn anchor_offset(&self) -> Vec2 {
        Vec2::new(
            self.local_bbox.x0 + self.local_bbox.width() * self.state.anchor.x,
            self.local_bbox.y0 + self.local_bbox.height() * self.state.anchor.y,
        )
    }

    fn recompute(&mut self) {
        let anchor = self.anchor_offset();
        self.transform = Affine::translate(self.state.position.to_vec2())
            * Affine::rotate(self.state.rotation)
            * Affine::scale_non_uniform(self.state.scale.x, self.state.scale.y)
            * Affine::translate(-Vec2::new(
                self.local_bbox.x0 + self.local_bbox.width() * self.state.anchor.x,
                self.local_bbox.y0 + self.local_bbox.height() * self.state.anchor.y,
            ));

        self.world_bbox = if self.is_identity() {
            self.local_bbox
        } else {
            let [a, b, c, d, e, f] = self.transform.as_coeffs();
            let local = self.local_bbox;

            let center_x = (local.x0 + local.x1) * 0.5;
            let center_y = (local.y0 + local.y1) * 0.5;
            let half_w = (local.x1 - local.x0) * 0.5;
            let half_h = (local.y1 - local.y0) * 0.5;

            let new_center_x = a * center_x + c * center_y + e;
            let new_center_y = b * center_x + d * center_y + f;
            let new_half_w = a.abs() * half_w + c.abs() * half_h;
            let new_half_h = b.abs() * half_w + d.abs() * half_h;

            Rect::new(
                new_center_x - new_half_w,
                new_center_y - new_half_h,
                new_center_x + new_half_w,
                new_center_y + new_half_h,
            )
        };

        self.dirty = false;
    }

    #[inline(always)]
    fn ensure_updated(&mut self) {
        if self.dirty {
            self.recompute();
        }
    }

    #[inline]
    pub fn transform(&mut self) -> Affine {
        self.ensure_updated();
        self.transform
    }

    #[inline]
    pub fn world_bounding_box(&mut self) -> Rect {
        self.ensure_updated();
        self.world_bbox
    }
}
