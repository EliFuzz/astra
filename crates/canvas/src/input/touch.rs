use kurbo::{Point, Vec2};
use winit::event::{Touch, TouchPhase};

use super::state::InputState;
use super::touch_state::TouchState;

impl InputState {
    pub fn process_touch(&mut self, touch: &Touch) -> Option<(Vec2, f64, Point)> {
        let pos = Point::new(touch.location.x, touch.location.y);
        let state = TouchState {
            id: touch.id,
            position: pos,
            phase: touch.phase,
        };

        match touch.phase {
            TouchPhase::Started => {
                if self.touches[0].is_none() {
                    self.touches[0] = Some(state);
                } else if self.touches[1].is_none() {
                    self.touches[1] = Some(state);
                    if let (Some(t0), Some(t1)) = (self.touches[0], self.touches[1]) {
                        self.pinch_distance = Some(t0.position.distance(t1.position));
                        self.pinch_center = Some(Point::new(
                            (t0.position.x + t1.position.x) / 2.0,
                            (t0.position.y + t1.position.y) / 2.0,
                        ));
                    }
                }
                None
            }
            TouchPhase::Moved => {
                for t in self.touches.iter_mut().flatten() {
                    if t.id == touch.id {
                        t.position = pos;
                        t.phase = touch.phase;
                    }
                }

                if let (Some(t0), Some(t1)) = (self.touches[0], self.touches[1]) {
                    let new_dist = t0.position.distance(t1.position);
                    let new_center = Point::new(
                        (t0.position.x + t1.position.x) / 2.0,
                        (t0.position.y + t1.position.y) / 2.0,
                    );

                    let zoom_delta = if let Some(old_dist) = self.pinch_distance {
                        if old_dist > 0.0 {
                            new_dist / old_dist
                        } else {
                            1.0
                        }
                    } else {
                        1.0
                    };

                    let pan_delta = if let Some(old_center) = self.pinch_center {
                        Vec2::new(new_center.x - old_center.x, new_center.y - old_center.y)
                    } else {
                        Vec2::ZERO
                    };

                    self.pinch_distance = Some(new_dist);
                    self.pinch_center = Some(new_center);

                    Some((pan_delta, zoom_delta, new_center))
                } else {
                    None
                }
            }
            TouchPhase::Ended | TouchPhase::Cancelled => {
                for slot in &mut self.touches {
                    if let Some(t) = slot {
                        if t.id == touch.id {
                            *slot = None;
                        }
                    }
                }
                self.pinch_distance = None;
                self.pinch_center = None;
                None
            }
        }
    }

    pub fn primary_touch(&self) -> Option<Point> {
        self.touches[0].map(|t| t.position)
    }

    pub fn touch_count(&self) -> usize {
        self.touches.iter().filter(|t| t.is_some()).count()
    }

    pub fn is_single_touch(&self) -> bool {
        self.touches[0].is_some() && self.touches[1].is_none()
    }

    pub fn touch_just_started(&self) -> bool {
        self.touches[0]
            .map(|t| t.phase == TouchPhase::Started)
            .unwrap_or(false)
    }

    pub fn touch_just_ended(&self) -> bool {
        self.touches[0].is_none() && self.touches[1].is_none()
    }
}
