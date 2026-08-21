use vello::kurbo::{self as K, Shape};

#[derive(Debug, Clone, Copy)]
pub enum AnyShape {
    Rect(K::Rect),
    Ellipse(K::Ellipse),
    Line(K::Line),
    Circle(K::Circle),
    PathSeg(K::PathSeg),
    RoundedRect(K::RoundedRect),
    Triangle(K::Triangle),
    Arc(K::Arc),
    CubicBez(K::CubicBez),
    QuadBez(K::QuadBez),
    CircleSegment(K::CircleSegment),
}

macro_rules! for_each_shape {
    ($self:expr, |$v:ident| $body:expr) => {
        match $self {
            AnyShape::Rect($v) => $body,
            AnyShape::Ellipse($v) => $body,
            AnyShape::Line($v) => $body,
            AnyShape::Circle($v) => $body,
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
    type PathElementsIter<'iter> = Box<dyn Iterator<Item = K::PathEl> + 'iter>;

    fn path_elements(&self, tolerance: f64) -> Self::PathElementsIter<'_> {
        for_each_shape!(self, |v| Box::new(v.path_elements(tolerance)))
    }

    fn area(&self) -> f64 {
        for_each_shape!(self, |v| v.area())
    }

    fn perimeter(&self, accuracy: f64) -> f64 {
        for_each_shape!(self, |v| v.perimeter(accuracy))
    }

    fn winding(&self, pt: K::Point) -> i32 {
        for_each_shape!(self, |v| v.winding(pt))
    }

    fn bounding_box(&self) -> K::Rect {
        for_each_shape!(self, |v| v.bounding_box())
    }

    fn to_path(&self, tolerance: f64) -> K::BezPath {
        for_each_shape!(self, |v| v.to_path(tolerance))
    }

    fn into_path(self, tolerance: f64) -> K::BezPath {
        for_each_shape!(self, |v| v.into_path(tolerance))
    }

    fn contains(&self, pt: K::Point) -> bool {
        for_each_shape!(self, |v| v.contains(pt))
    }

    fn as_line(&self) -> Option<K::Line> {
        for_each_shape!(self, |v| v.as_line())
    }

    fn as_rect(&self) -> Option<K::Rect> {
        for_each_shape!(self, |v| v.as_rect())
    }

    fn as_rounded_rect(&self) -> Option<K::RoundedRect> {
        for_each_shape!(self, |v| v.as_rounded_rect())
    }

    fn as_circle(&self) -> Option<K::Circle> {
        for_each_shape!(self, |v| v.as_circle())
    }

    fn as_path_slice(&self) -> Option<&[K::PathEl]> {
        for_each_shape!(self, |v| v.as_path_slice())
    }
}

// macro_rules! any_shape {
//     ($shape:expr) => {
//         AnyShape::from($shape)
//     };
// }

impl From<K::Rect> for AnyShape {
    fn from(shape: K::Rect) -> Self {
        AnyShape::Rect(shape)
    }
}

impl From<K::Ellipse> for AnyShape {
    fn from(shape: K::Ellipse) -> Self {
        AnyShape::Ellipse(shape)
    }
}

impl From<K::Line> for AnyShape {
    fn from(shape: K::Line) -> Self {
        AnyShape::Line(shape)
    }
}

impl From<K::Circle> for AnyShape {
    fn from(shape: K::Circle) -> Self {
        AnyShape::Circle(shape)
    }
}

impl From<K::PathSeg> for AnyShape {
    fn from(shape: K::PathSeg) -> Self {
        AnyShape::PathSeg(shape)
    }
}

impl From<K::RoundedRect> for AnyShape {
    fn from(shape: K::RoundedRect) -> Self {
        AnyShape::RoundedRect(shape)
    }
}

impl From<K::Triangle> for AnyShape {
    fn from(shape: K::Triangle) -> Self {
        AnyShape::Triangle(shape)
    }
}

impl From<K::Arc> for AnyShape {
    fn from(shape: K::Arc) -> Self {
        AnyShape::Arc(shape)
    }
}

impl From<K::CubicBez> for AnyShape {
    fn from(shape: K::CubicBez) -> Self {
        AnyShape::CubicBez(shape)
    }
}

impl From<K::QuadBez> for AnyShape {
    fn from(shape: K::QuadBez) -> Self {
        AnyShape::QuadBez(shape)
    }
}

impl From<K::CircleSegment> for AnyShape {
    fn from(shape: K::CircleSegment) -> Self {
        AnyShape::CircleSegment(shape)
    }
}
