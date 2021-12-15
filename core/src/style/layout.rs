
// Not currently used

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Overflow {
    Visible,
    Hidden,
}

impl Default for Overflow {
    fn default() -> Self {
        Overflow::Hidden
    }
}

// #[derive(Copy, Clone, Debug, PartialEq)]
// pub struct Scroll {
//     pub x: f32,
//     pub y: f32,
//     pub w: f32,
//     pub h: f32,
// }

// impl Default for Scroll {
//     fn default() -> Self {
//         Scroll {
//             x: 0.0,
//             y: 0.0,
//             w: 1.0,
//             h: 1.0,
//         }
//     }
// }
