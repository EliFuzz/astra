use egui::Color32;

#[derive(Clone, Copy)]
pub struct Color {
    pub name: &'static str,
    pub shades: [Color32; 12],
}

impl Color {
    pub const fn new(name: &'static str, shades: [(u8, u8, u8); 12]) -> Self {
        Self {
            name,
            shades: [
                Color32::from_rgb(shades[0].0, shades[0].1, shades[0].2),
                Color32::from_rgb(shades[1].0, shades[1].1, shades[1].2),
                Color32::from_rgb(shades[2].0, shades[2].1, shades[2].2),
                Color32::from_rgb(shades[3].0, shades[3].1, shades[3].2),
                Color32::from_rgb(shades[4].0, shades[4].1, shades[4].2),
                Color32::from_rgb(shades[5].0, shades[5].1, shades[5].2),
                Color32::from_rgb(shades[6].0, shades[6].1, shades[6].2),
                Color32::from_rgb(shades[7].0, shades[7].1, shades[7].2),
                Color32::from_rgb(shades[8].0, shades[8].1, shades[8].2),
                Color32::from_rgb(shades[9].0, shades[9].1, shades[9].2),
                Color32::from_rgb(shades[10].0, shades[10].1, shades[10].2),
                Color32::from_rgb(shades[11].0, shades[11].1, shades[11].2),
            ],
        }
    }

    pub const fn primary(&self) -> Color32 {
        self.shades[6]
    }

    pub const fn shade(&self, index: usize) -> Color32 {
        self.shades[index]
    }
}
