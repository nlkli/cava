use vello::Scene;
use vello::kurbo::{Affine, Ellipse, Line, Point, Rect, Shape, Stroke};
use vello::peniko::{Color, Fill};

use crate::camera::Camera;

/// The basic kurbo shapes a [`ShapeElement`] can hold.
#[derive(Debug, Clone)]
pub enum Geometry {
    Rect(Rect),
    Ellipse(Ellipse),
    Line(Line),
}

impl Geometry {
    /// The untransformed bounding box of the geometry.
    fn local_bounds(&self) -> Rect {
        match self {
            Geometry::Rect(r) => r.bounding_box(),
            Geometry::Ellipse(e) => e.bounding_box(),
            Geometry::Line(l) => l.bounding_box(),
        }
    }
}

/// Fill and/or stroke appearance for a [`ShapeElement`].
#[derive(Debug, Clone, Copy)]
pub struct Style {
    pub fill: Option<Color>,
    pub stroke: Option<(Color, f64)>,
}

impl Style {
    /// A solid fill with no stroke.
    pub fn filled(color: Color) -> Self {
        Self {
            fill: Some(color),
            stroke: None,
        }
    }

    /// A stroke of the given width with no fill.
    pub fn stroked(color: Color, width: f64) -> Self {
        Self {
            fill: None,
            stroke: Some((color, width)),
        }
    }

    /// Both a fill and a stroke.
    pub fn filled_and_stroked(fill: Color, stroke: Color, width: f64) -> Self {
        Self {
            fill: Some(fill),
            stroke: Some((stroke, width)),
        }
    }
}

/// A single drawable shape placed in world space via a local transform.
#[derive(Debug, Clone)]
pub struct ShapeElement {
    geometry: Geometry,
    style: Style,
    transform: Affine,
}

impl ShapeElement {
    /// Creates a new element at the identity transform (i.e. in the
    /// geometry's own coordinates, unmodified).
    pub fn new(geometry: Geometry, style: Style) -> Self {
        Self {
            geometry,
            style,
            transform: Affine::IDENTITY,
        }
    }

    /// Places the element in world space via a local transform (builder-style).
    pub fn with_transform(mut self, transform: Affine) -> Self {
        self.transform = transform;
        self
    }
}

impl Element for ShapeElement {
    fn bounds(&self) -> Rect {
        let local = self.geometry.local_bounds();
        // Transform all four corners individually (not just min/max) so
        // rotation or skew in `transform` still yields a correct AABB.
        let corners = [
            self.transform * Point::new(local.x0, local.y0),
            self.transform * Point::new(local.x1, local.y0),
            self.transform * Point::new(local.x1, local.y1),
            self.transform * Point::new(local.x0, local.y1),
        ];
        Rect::from_points(corners[0], corners[1])
            .union_pt(corners[2])
            .union_pt(corners[3])
    }

    fn draw(&self, scene: &mut Scene, view: Affine) {
        let transform = view * self.transform;

        if let Some(fill_color) = self.style.fill {
            match &self.geometry {
                Geometry::Rect(r) => scene.fill(Fill::NonZero, transform, fill_color, None, r),
                Geometry::Ellipse(e) => scene.fill(Fill::NonZero, transform, fill_color, None, e),
                Geometry::Line(_) => {} // a line has no interior to fill
            }
        }

        if let Some((stroke_color, width)) = self.style.stroke {
            let stroke = Stroke::new(width);
            match &self.geometry {
                Geometry::Rect(r) => scene.stroke(&stroke, transform, stroke_color, None, r),
                Geometry::Ellipse(e) => scene.stroke(&stroke, transform, stroke_color, None, e),
                Geometry::Line(l) => scene.stroke(&stroke, transform, stroke_color, None, l),
            }
        }
    }
}

/// Something that can report its world-space bounds and draw itself into
/// a [`Scene`], given the camera's world-to-screen transform.
pub trait Element: std::fmt::Debug {
    /// The element's axis-aligned bounding box in world space, used for
    /// culling. Must fully contain the drawn geometry.
    fn bounds(&self) -> Rect;

    /// Draws the element into `scene`. `view` is the camera's
    /// world-to-screen transform; implementations compose it with any
    /// element-local transform before issuing drawing commands.
    fn draw(&self, scene: &mut Scene, view: Affine);
}

// ---------------------------------------------------------------------
// Document
// ---------------------------------------------------------------------

/// A flat collection of drawable elements, rendered with viewport culling.
#[derive(Default)]
pub struct Document {
    pub elements: Vec<Box<dyn Element>>,
}

impl std::fmt::Debug for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Document")
            .field("element_count", &self.elements.len())
            .finish()
    }
}

impl Document {
    /// Creates an empty document.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an element to the document.
    pub fn add(&mut self, element: impl Element + 'static) -> &mut Self {
        self.elements.push(Box::new(element));
        self
    }

    /// Draws every element whose bounds intersect the camera's visible
    /// world rectangle, skipping (culling) all others.
    pub fn render(&self, scene: &mut Scene, camera: &Camera) {
        let visible = camera.visible_world_rect();
        let view = camera.world_to_screen_affine();

        for element in &self.elements {
            let intersection = element.bounds().intersect(visible);
            // Check if the intersection has zero or negative width/height
            if intersection.width() <= 0.0 || intersection.height() <= 0.0 {
                continue;
            }
            element.draw(scene, view);
        }
    }
}
