use vello::Scene;
use vello::kurbo::{
    Affine, Arc, BezPath, Circle, CircleSegment, CubicBez, Ellipse, Line, PathEl, PathSeg, Point,
    QuadBez, Rect, RoundedRect, Shape, Stroke, Triangle, Vec2,
};
use vello::peniko::{Color, Fill};

#[derive(Debug, Clone, Default)]
pub struct Style {
    pub fill: Option<Color>,
    pub stroke: Option<(Color, Stroke)>,
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn filled(color: Color) -> Self {
        Self {
            fill: Some(color),
            stroke: None,
        }
    }
    pub fn stroked(color: Color, width: f64) -> Self {
        Self {
            fill: None,
            stroke: Some((color, Stroke::new(width))),
        }
    }
    pub fn filled_and_stroked(fill: Color, stroke: Color, width: f64) -> Self {
        Self {
            fill: Some(fill),
            stroke: Some((stroke, Stroke::new(width))),
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.set_fill_color(color);
        self.set_stroke_color(color);
    }

    pub fn set_fill_color(&mut self, color: Color) {
        if let Some(fill) = &mut self.fill {
            *fill = color;
        }
    }

    pub fn set_stroke_color(&mut self, color: Color) {
        if let Some((stroke_color, _)) = &mut self.stroke {
            *stroke_color = color;
        }
    }
}

#[derive(Debug, Clone)]
pub enum AnyShape {
    Rect(Rect),
    Ellipse(Ellipse),
    Line(Line),
    Circle(Circle),
    BezPath(BezPath),
    PathSeg(PathSeg),
    RoundedRect(RoundedRect),
    Triangle(Triangle),
    Arc(Arc),
    CubicBez(CubicBez),
    QuadBez(QuadBez),
    CircleSegment(CircleSegment),
}

macro_rules! for_each_shape {
    ($self:expr, |$v:ident| $body:expr) => {
        match $self {
            AnyShape::Rect($v) => $body,
            AnyShape::Ellipse($v) => $body,
            AnyShape::Line($v) => $body,
            AnyShape::Circle($v) => $body,
            AnyShape::BezPath($v) => $body,
            AnyShape::PathSeg($v) => $body,
            AnyShape::RoundedRect($v) => $body,
            AnyShape::Triangle($v) => $body,
            AnyShape::Arc($v) => $body,
            AnyShape::CubicBez($v) => $body,
            AnyShape::QuadBez($v) => $body,
            AnyShape::CircleSegment($v) => $body,
        }
    };
}

impl Shape for AnyShape {
    type PathElementsIter<'iter> = Box<dyn Iterator<Item = PathEl> + 'iter>;

    fn path_elements(&self, tolerance: f64) -> Self::PathElementsIter<'_> {
        for_each_shape!(self, |v| Box::new(v.path_elements(tolerance)))
    }

    fn area(&self) -> f64 {
        for_each_shape!(self, |v| v.area())
    }

    fn perimeter(&self, accuracy: f64) -> f64 {
        for_each_shape!(self, |v| v.perimeter(accuracy))
    }

    fn winding(&self, pt: Point) -> i32 {
        for_each_shape!(self, |v| v.winding(pt))
    }

    fn bounding_box(&self) -> Rect {
        for_each_shape!(self, |v| v.bounding_box())
    }

    fn to_path(&self, tolerance: f64) -> BezPath {
        for_each_shape!(self, |v| v.to_path(tolerance))
    }

    fn into_path(self, tolerance: f64) -> BezPath {
        for_each_shape!(self, |v| v.into_path(tolerance))
    }

    fn contains(&self, pt: Point) -> bool {
        for_each_shape!(self, |v| v.contains(pt))
    }

    fn as_line(&self) -> Option<Line> {
        for_each_shape!(self, |v| v.as_line())
    }

    fn as_rect(&self) -> Option<Rect> {
        for_each_shape!(self, |v| v.as_rect())
    }

    fn as_rounded_rect(&self) -> Option<RoundedRect> {
        for_each_shape!(self, |v| v.as_rounded_rect())
    }

    fn as_circle(&self) -> Option<Circle> {
        for_each_shape!(self, |v| v.as_circle())
    }

    fn as_path_slice(&self) -> Option<&[PathEl]> {
        for_each_shape!(self, |v| v.as_path_slice())
    }
}

// macro_rules! any_shape {
//     ($shape:expr) => {
//         AnyShape::from($shape)
//     };
// }

impl From<Rect> for AnyShape {
    fn from(shape: Rect) -> Self {
        AnyShape::Rect(shape)
    }
}

impl From<Ellipse> for AnyShape {
    fn from(shape: Ellipse) -> Self {
        AnyShape::Ellipse(shape)
    }
}

impl From<Line> for AnyShape {
    fn from(shape: Line) -> Self {
        AnyShape::Line(shape)
    }
}

impl From<Circle> for AnyShape {
    fn from(shape: Circle) -> Self {
        AnyShape::Circle(shape)
    }
}

impl From<BezPath> for AnyShape {
    fn from(shape: BezPath) -> Self {
        AnyShape::BezPath(shape)
    }
}

impl From<PathSeg> for AnyShape {
    fn from(shape: PathSeg) -> Self {
        AnyShape::PathSeg(shape)
    }
}

impl From<RoundedRect> for AnyShape {
    fn from(shape: RoundedRect) -> Self {
        AnyShape::RoundedRect(shape)
    }
}

impl From<Triangle> for AnyShape {
    fn from(shape: Triangle) -> Self {
        AnyShape::Triangle(shape)
    }
}

impl From<Arc> for AnyShape {
    fn from(shape: Arc) -> Self {
        AnyShape::Arc(shape)
    }
}

impl From<CubicBez> for AnyShape {
    fn from(shape: CubicBez) -> Self {
        AnyShape::CubicBez(shape)
    }
}

impl From<QuadBez> for AnyShape {
    fn from(shape: QuadBez) -> Self {
        AnyShape::QuadBez(shape)
    }
}

impl From<CircleSegment> for AnyShape {
    fn from(shape: CircleSegment) -> Self {
        AnyShape::CircleSegment(shape)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ElemState {
    pub position: Point,
    pub rotation: f64,
    pub scale: Vec2,
    pub anchor: Vec2,
}

impl Default for ElemState {
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
pub struct Elem {
    pub style: Style,
    shape: AnyShape,
    /// Bounding box of `shape` in local (untransformed) space.
    local_bbox: Rect,
    /// Cached `local_bbox` transformed into world space. Valid iff `!dirty`.
    world_bbox: Rect,
    state: ElemState,
    transform: Affine,
    /// Set whenever `state` or `shape` changes; cleared by `recompute`.
    dirty: bool,
}

impl Default for Elem {
    fn default() -> Self {
        Elem::new(Rect::default().into(), Style::default(), None)
    }
}

impl Elem {
    pub fn new(shape: AnyShape, style: Style, state: Option<ElemState>) -> Self {
        let local_bbox = shape.bounding_box();
        Self {
            style,
            shape,
            local_bbox,
            world_bbox: local_bbox, // overwritten on first ensure_updated(), dirty = true
            state: state.unwrap_or_default(),
            transform: Affine::IDENTITY,
            dirty: true,
        }
    }

    pub fn builder() -> ElemBuilder {
        ElemBuilder::new()
    }

    #[inline(always)]
    pub fn state(&self) -> &ElemState {
        &self.state
    }

    #[inline(always)]
    pub fn state_mut(&mut self) -> &mut ElemState {
        self.dirty = true;
        &mut self.state
    }

    #[inline(always)]
    pub fn shape(&self) -> AnyShape {
        self.shape.clone()
    }

    #[inline(always)]
    pub fn set_shape(&mut self, shape: AnyShape) {
        self.local_bbox = shape.bounding_box();
        self.shape = shape;
        self.dirty = true;
    }

    #[inline(always)]
    fn anchor_offset(&self) -> Vec2 {
        Vec2::new(
            self.local_bbox.x0 + self.local_bbox.width() * self.state.anchor.x,
            self.local_bbox.y0 + self.local_bbox.height() * self.state.anchor.y,
        )
    }

    // Cheap check instead of unreliable float equality with Affine::IDENTITY.
    #[inline(always)]
    fn is_identity(&self) -> bool {
        self.state.rotation == 0.0
            && self.state.scale == Vec2::new(1.0, 1.0)
            && self.state.position == Point::ZERO
    }

    /// Recomputes `transform` and `world_bbox` together — they share the same
    /// invalidation events, so keeping them in one place rules out desync
    /// between the two caches.
    fn recompute(&mut self) {
        let anchor = self.anchor_offset();
        self.transform = Affine::translate(self.state.position.to_vec2())
            * Affine::rotate(self.state.rotation)
            * Affine::scale_non_uniform(self.state.scale.x, self.state.scale.y)
            * Affine::translate(-anchor);

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

    /// O(1) when nothing changed since the last call — the world-space
    /// bounding box is cached alongside the transform.
    #[inline]
    pub fn world_bounding_box(&mut self) -> Rect {
        self.ensure_updated();
        self.world_bbox
    }

    // Must be &mut to guarantee the cached transform reflects current state
    // before it's read directly below — this was a real bug (stale transform bypass).
    #[inline]
    pub fn render(&mut self, scene: &mut Scene) {
        self.render_with_base(scene, Affine::IDENTITY);
    }

    pub fn render_with_base(&mut self, scene: &mut Scene, base: Affine) {
        let transform = base * self.transform(); // forces recompute if dirty
        if let Some(color) = self.style.fill {
            scene.fill(Fill::NonZero, transform, color, None, &self.shape);
        }
        if let Some((color, stroke)) = &self.style.stroke {
            scene.stroke(stroke, transform, color, None, &self.shape);
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ElemBuilder {
    el: Elem,
}

impl ElemBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_shape(mut self, shape: impl Into<AnyShape>) -> Self {
        self.el.set_shape(shape.into());
        self
    }

    pub fn with_style(mut self, style: Style) -> Self {
        self.el.style = style;
        self
    }

    pub fn with_state(mut self, state: ElemState) -> Self {
        self.el.state = state;
        self
    }

    pub fn with_position(mut self, position: Point) -> Self {
        self.el.state.position = position;
        self
    }

    pub fn with_position_point(mut self, x: f64, y: f64) -> Self {
        self.el.state.position = Point::new(x, y);
        self
    }

    pub fn with_rotation(mut self, rotation: f64) -> Self {
        self.el.state.rotation = rotation;
        self
    }

    pub fn with_scale(mut self, scale: Vec2) -> Self {
        self.el.state.scale = scale;
        self
    }

    pub fn with_scale_values(mut self, x: f64, y: f64) -> Self {
        self.el.state.scale = Vec2::new(x, y);
        self
    }

    pub fn with_anchor(mut self, anchor: Vec2) -> Self {
        self.el.state.anchor = anchor;
        self
    }

    pub fn with_anchor_values(mut self, x: f64, y: f64) -> Self {
        self.el.state.anchor = Vec2::new(x, y);
        self
    }

    pub fn build(self) -> Elem {
        self.el
    }
}
