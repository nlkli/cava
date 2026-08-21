use vello::{kurbo::Stroke, peniko::Color};

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

    #[inline(always)]
    pub fn set_fill_color(&mut self, color: Color) {
        if let Some(fill) = &mut self.fill {
            *fill = color;
        }
    }

    #[inline(always)]
    pub fn set_stroke_color(&mut self, color: Color) {
        if let Some((stroke_color, _)) = &mut self.stroke {
            *stroke_color = color;
        }
    }
}
