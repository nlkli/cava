use vello::{
    kurbo::{Affine, BezPath, Circle, Ellipse, Line, PathEl, Point, Rect, RoundedRect, Shape, Stroke, Vec2}, peniko::Color,
};

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
}

macro_rules! for_each_shape {
    ($self:expr, |$v:ident| $body:expr) => {
        match $self {
            AnyShape::Rect($v) => $body,
            AnyShape::Ellipse($v) => $body,
            AnyShape::Line($v) => $body,
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

pub struct ElementState {
    position: Point,
    rotation: f64,
    scale: f64,

    transform: Affine,
}

pub enum ElementType {
    Shape { value: AnyShape, style: Style },
    Text { value: String },
    Group { value: Vec<Element> },
}

pub struct Element {
    id: u64,
    r#type: ElementType,

    position: Point,
    rotation: f64,
    scale: f64,

    transform: Affine,
}

pub struct Document {
    elements: Vec<Element>,
    id_acc: u64,
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
