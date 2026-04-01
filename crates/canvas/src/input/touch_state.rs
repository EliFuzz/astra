use kurbo::Point;
use winit::event::TouchPhase;

#[derive(Debug, Clone, Copy)]
pub struct TouchState {
    pub id: u64,
    pub position: Point,
    pub phase: TouchPhase,
}
