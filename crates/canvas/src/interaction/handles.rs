use kurbo::Point;

#[derive(Debug, Clone)]
pub struct Handle {
    pub kind: HandleKind,
    pub position: Point,
    pub shape: HandleShape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HandleKind {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Top,
    Bottom,
    Left,
    Right,
    Start,
    End,
    Control(usize),
    Rotate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HandleShape {
    #[default]
    Square,
    Circle,
    Diamond,
}

impl Handle {
    pub fn new(kind: HandleKind, position: Point) -> Self {
        Self {
            kind,
            position,
            shape: HandleShape::default(),
        }
    }

    pub fn with_shape(mut self, shape: HandleShape) -> Self {
        self.shape = shape;
        self
    }
}
