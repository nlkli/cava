use vello::Scene;
use vello::kurbo::{
    Affine, Arc, BezPath, Circle, CircleSegment, CubicBez, Ellipse, Line, PathEl, PathSeg, Point,
    QuadBez, Rect, RoundedRect, Shape, Stroke, Triangle,
};
use vello::peniko::{Color, Fill};

#[derive(Debug, Clone, Copy)]
pub struct Style {
    pub fill: Option<Color>,
    pub stroke: Option<(Color, f64)>,
}

impl Style {
    pub fn filled(color: Color) -> Self {
        Self {
            fill: Some(color),
            stroke: None,
        }
    }
    pub fn stroked(color: Color, width: f64) -> Self {
        Self {
            fill: None,
            stroke: Some((color, width)),
        }
    }
    pub fn filled_and_stroked(fill: Color, stroke: Color, width: f64) -> Self {
        Self {
            fill: Some(fill),
            stroke: Some((stroke, width)),
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

pub struct El {
    pub shape: AnyShape,
    pub style: Style,
    pub transform: Affine,
}

impl El {
    pub fn new(shape: AnyShape, style: Style, transform: Option<Affine>) -> Self {
        Self {
            shape,
            style,
            transform: transform.unwrap_or(Affine::IDENTITY),
        }
    }

    pub fn world_bounding_box(&self) -> Rect {
        let local = self.shape.bounding_box();
        let pts = [
            self.transform * Point::new(local.x0, local.y0),
            self.transform * Point::new(local.x1, local.y0),
            self.transform * Point::new(local.x1, local.y1),
            self.transform * Point::new(local.x0, local.y1),
        ];
        let (mut x0, mut y0) = (f64::INFINITY, f64::INFINITY);
        let (mut x1, mut y1) = (f64::NEG_INFINITY, f64::NEG_INFINITY);
        for p in pts {
            x0 = x0.min(p.x);
            y0 = y0.min(p.y);
            x1 = x1.max(p.x);
            y1 = y1.max(p.y);
        }
        Rect::new(x0, y0, x1, y1)
    }

    pub fn render(&self, scene: &mut Scene) {
        self.render_with_base(scene, Affine::IDENTITY);
    }

    pub fn render_with_base(&self, scene: &mut Scene, base: Affine) {
        let transform = base * self.transform;
        if let Some(color) = self.style.fill {
            scene.fill(Fill::NonZero, transform, color, None, &self.shape);
        }
        if let Some((color, width)) = self.style.stroke {
            let stroke = Stroke::new(width);
            scene.stroke(&stroke, transform, color, None, &self.shape);
        }
    }
}

pub fn rect(rect: Rect, style: Style, transform: Option<Affine>) -> El {
    El::new(AnyShape::Rect(rect), style, transform)
}
pub fn ellipse(ellipse: Ellipse, style: Style, transform: Option<Affine>) -> El {
    El::new(AnyShape::Ellipse(ellipse), style, transform)
}
pub fn line(line: Line, style: Style, transform: Option<Affine>) -> El {
    El::new(AnyShape::Line(line), style, transform)
}
pub fn circle(circle: Circle, style: Style, transform: Option<Affine>) -> El {
    El::new(AnyShape::Circle(circle), style, transform)
}
pub fn bez_path(path: BezPath, style: Style, transform: Option<Affine>) -> El {
    El::new(AnyShape::BezPath(path), style, transform)
}
pub fn path_seg(seg: PathSeg, style: Style, transform: Option<Affine>) -> El {
    El::new(AnyShape::PathSeg(seg), style, transform)
}
pub fn rounded_rect(rect: RoundedRect, style: Style, transform: Option<Affine>) -> El {
    El::new(AnyShape::RoundedRect(rect), style, transform)
}
pub fn triangle(tri: Triangle, style: Style, transform: Option<Affine>) -> El {
    El::new(AnyShape::Triangle(tri), style, transform)
}
pub fn arc(arc: Arc, style: Style, transform: Option<Affine>) -> El {
    El::new(AnyShape::Arc(arc), style, transform)
}
pub fn cubic_bez(cubic: CubicBez, style: Style, transform: Option<Affine>) -> El {
    El::new(AnyShape::CubicBez(cubic), style, transform)
}
pub fn quad_bez(quad: QuadBez, style: Style, transform: Option<Affine>) -> El {
    El::new(AnyShape::QuadBez(quad), style, transform)
}
pub fn circle_segment(seg: CircleSegment, style: Style, transform: Option<Affine>) -> El {
    El::new(AnyShape::CircleSegment(seg), style, transform)
}
