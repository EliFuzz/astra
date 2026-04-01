use super::VelloRenderer;
use super::effects::apply_hand_drawn_effect;
use kurbo::{BezPath, PathEl};

pub(crate) type GlyphRun = (
    vello::peniko::FontData,
    f32,
    peniko::Brush,
    Vec<vello::Glyph>,
    Option<f64>,
);

#[derive(Debug)]
pub struct PngRenderResult {
    pub rgba_data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone)]
pub(crate) struct CachedTextLayout {
    pub(crate) glyph_runs: Vec<GlyphRun>,
    pub(crate) width: f64,
    pub(crate) height: f64,
    pub(crate) access_gen: u64,
}

const FNV_OFFSET: u64 = 14695981039346656037;
const FNV_PRIME: u64 = 1099511628211;

#[inline]
fn fnv_mix(h: u64, v: u64) -> u64 {
    (h ^ v).wrapping_mul(FNV_PRIME)
}

fn path_element_hash(path: &BezPath) -> u64 {
    let mut h = FNV_OFFSET;
    for el in path.elements() {
        match el {
            PathEl::MoveTo(p) | PathEl::LineTo(p) => {
                h = fnv_mix(h, 0x4D4C);
                h = fnv_mix(h, p.x.to_bits());
                h = fnv_mix(h, p.y.to_bits());
            }
            PathEl::QuadTo(p1, p2) => {
                h = fnv_mix(h, 0x5154);
                h = fnv_mix(h, p1.x.to_bits());
                h = fnv_mix(h, p1.y.to_bits());
                h = fnv_mix(h, p2.x.to_bits());
                h = fnv_mix(h, p2.y.to_bits());
            }
            PathEl::CurveTo(p1, p2, p3) => {
                h = fnv_mix(h, 0x4356);
                h = fnv_mix(h, p1.x.to_bits());
                h = fnv_mix(h, p1.y.to_bits());
                h = fnv_mix(h, p2.x.to_bits());
                h = fnv_mix(h, p2.y.to_bits());
                h = fnv_mix(h, p3.x.to_bits());
                h = fnv_mix(h, p3.y.to_bits());
            }
            PathEl::ClosePath => {
                h = fnv_mix(h, 0x434C);
            }
        }
    }
    h
}

fn zoom_bucket(zoom: f64) -> i32 {
    let z = zoom.max(f64::MIN_POSITIVE);
    let scaled = z.log2() * 4.0;
    if scaled <= i32::MIN as f64 {
        i32::MIN
    } else if scaled >= i32::MAX as f64 {
        i32::MAX
    } else {
        scaled.round() as i32
    }
}

impl VelloRenderer {
    pub(crate) fn get_cached_hand_drawn(
        &mut self,
        shape_id: &str,
        path: &BezPath,
        roughness: f64,
        seed: u32,
        stroke_index: u32,
    ) -> BezPath {
        self.generation = self.generation.wrapping_add(1);
        let touch = self.generation;
        let zb = zoom_bucket(self.zoom);
        let roughness_bits = roughness.to_bits();
        let path_hash = path_element_hash(path);
        let key = (
            shape_id.to_string(),
            seed,
            stroke_index,
            roughness_bits,
            zb,
            path_hash,
        );

        if let Some(entry) = self.shape_cache.get_mut(&key) {
            entry.1 = touch;
            return entry.0.clone();
        }

        let result = apply_hand_drawn_effect(path, roughness, self.zoom, seed, stroke_index);
        self.shape_cache.insert(key, (result.clone(), touch));

        if self.shape_cache.len() > 1000 {
            let mut entries: Vec<_> = self.shape_cache.drain().collect();
            entries.sort_unstable_by(|a, b| b.1.1.cmp(&a.1.1));
            entries.truncate(500);
            self.shape_cache.extend(entries);
        }

        result
    }
}
