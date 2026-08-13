use vello::kurbo::{Affine, Point, Rect, Size, Vec2};

// ---------------------------------------------------------------------
// Camera
// ---------------------------------------------------------------------

/// A 2D camera mapping world-space coordinates to screen-space pixels.
///
/// The camera is defined by a world-space `position` (the point mapped to
/// the center of the viewport), a uniform `zoom`, and a `rotation`. Pan,
/// zoom, and rotation all compose into a single [`Affine`] transform used
/// both for drawing and for hit-testing/culling.
///
/// Zooming and rotating are anchored at an arbitrary *screen* point (e.g.
/// the cursor), meaning the world point under that screen point stays
/// visually fixed - the behavior expected of any professional 2D editor.
#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub position: Point,
    pub zoom: f64,
    pub rotation: f64,
    pub viewport: Size,
}

impl Camera {
    /// Creates a camera centered at the world origin with no zoom or
    /// rotation, sized to `viewport` screen pixels.
    pub fn new(viewport: Size) -> Self {
        Self {
            position: Point::ORIGIN,
            zoom: 1.0,
            rotation: 0.0,
            viewport,
        }
    }

    /// Translates the camera by a world-space delta.
    pub fn pan(&mut self, world_delta: Vec2) -> &mut Self {
        self.position = self.position + world_delta;
        self
    }

    /// Translates the camera so the view shifts by a screen-space delta
    /// (e.g. a mouse drag), regardless of current zoom/rotation.
    pub fn pan_screen(&mut self, screen_delta: Vec2) -> &mut Self {
        let world_delta = Self::apply_linear(self.view_linear().inverse(), screen_delta);
        self.position = self.position - world_delta;
        self
    }

    /// Multiplies the zoom by `factor`, clamped to the configured limits,
    /// keeping the world point under `screen_anchor` visually fixed.
    pub fn zoom_at(&mut self, screen_anchor: Point, factor: f64) -> &mut Self {
        let world_anchor = self.screen_to_world(screen_anchor);
        self.zoom = self.zoom * factor;
        self.position = self.solve_position_for_anchor(screen_anchor, world_anchor);
        self
    }

    /// Rotates the view by `delta_angle` radians, keeping the world point
    /// under `screen_pivot` visually fixed.
    pub fn rotate_around(&mut self, screen_pivot: Point, delta_angle: f64) -> &mut Self {
        let world_anchor = self.screen_to_world(screen_pivot);
        self.rotation += delta_angle;
        self.position = self.solve_position_for_anchor(screen_pivot, world_anchor);
        self
    }

    /// The full world-to-screen transform: pan, zoom, and rotation combined.
    pub fn world_to_screen_affine(&self) -> Affine {
        Affine::translate(self.screen_center().to_vec2())
            * self.view_linear()
            * Affine::translate(-self.position.to_vec2())
    }

    /// The full screen-to-world transform (inverse of [`Self::world_to_screen_affine`]).
    pub fn screen_to_world_affine(&self) -> Affine {
        self.world_to_screen_affine().inverse()
    }

    /// Maps a world-space point to screen space.
    pub fn world_to_screen(&self, point: Point) -> Point {
        self.world_to_screen_affine() * point
    }

    /// Maps a screen-space point to world space.
    pub fn screen_to_world(&self, point: Point) -> Point {
        self.screen_to_world_affine() * point
    }

    /// The axis-aligned world-space rectangle covering everything visible
    /// in the viewport. Conservative under rotation (covers the rotated
    /// viewport's bounding box), suitable for culling.
    pub fn visible_world_rect(&self) -> Rect {
        let to_world = self.screen_to_world_affine();
        let corners = [
            to_world * Point::new(0.0, 0.0),
            to_world * Point::new(self.viewport.width, 0.0),
            to_world * Point::new(self.viewport.width, self.viewport.height),
            to_world * Point::new(0.0, self.viewport.height),
        ];
        Rect::from_points(corners[0], corners[1])
            .union_pt(corners[2])
            .union_pt(corners[3])
    }

    /// The screen-space center of the viewport.
    fn screen_center(&self) -> Point {
        Point::new(self.viewport.width * 0.5, self.viewport.height * 0.5)
    }

    /// The rotation+zoom part of the camera transform, with no translation
    /// (i.e. it fixes the origin). Used to convert directions/deltas
    /// between world and screen space independent of camera position.
    fn view_linear(&self) -> Affine {
        Affine::rotate(self.rotation) * Affine::scale(self.zoom)
    }

    /// Applies a purely linear (translation-free) affine to a vector.
    fn apply_linear(transform: Affine, v: Vec2) -> Vec2 {
        (transform * Point::new(v.x, v.y)).to_vec2()
    }

    /// Solves for the camera position that keeps `world_anchor` mapped to
    /// `screen_anchor` under the camera's *current* zoom/rotation. Used
    /// after changing zoom or rotation to preserve the anchor point.
    fn solve_position_for_anchor(&self, screen_anchor: Point, world_anchor: Point) -> Point {
        let offset = Self::apply_linear(
            self.view_linear().inverse(),
            screen_anchor - self.screen_center(),
        );
        world_anchor - offset
    }
}
