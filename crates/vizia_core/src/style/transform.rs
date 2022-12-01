use std::ops::{Index, IndexMut};

use vizia_style::Transform;

use crate::{cache::BoundingBox, systems::transform};

/// A 2D transform matrix.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Transform2D(pub [f32; 6]);

impl Transform2D {
    pub fn new(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) -> Self {
        Self([a, b, c, d, e, f])
    }

    pub fn from_style_transforms(transforms: &Vec<Transform>, parent_bounds: BoundingBox) -> Self {
        let mut result = Transform2D::identity();
        for transform in transforms.iter() {
            let t = match transform {
                Transform::Translate(translate) => {
                    let tx = translate.x.to_pixels(parent_bounds.w);
                    let ty = translate.y.to_pixels(parent_bounds.h);

                    Transform2D::with_translate(tx, ty)
                }

                Transform::TranslateX(x) => {
                    let tx = x.to_pixels(parent_bounds.h);

                    Transform2D::with_translate(tx, 0.0)
                }

                Transform::TranslateY(y) => {
                    let ty = y.to_pixels(parent_bounds.h);

                    Transform2D::with_translate(0.0, ty)
                }

                Transform::Scale(scale) => {
                    let sx = scale.x.to_factor();
                    let sy = scale.y.to_factor();

                    Transform2D::with_scale(sx, sy)
                }

                Transform::ScaleX(x) => {
                    let sx = x.to_factor();

                    Transform2D::with_scale(sx, 1.0)
                }

                Transform::ScaleY(y) => {
                    let sy = y.to_factor();

                    Transform2D::with_scale(1.0, sy)
                }

                Transform::Rotate(angle) => Transform2D::with_rotate(angle.to_radians()),

                Transform::Skew(x, y) => {
                    let cx = x.to_radians().tan();
                    let cy = y.to_radians().tan();

                    Transform2D::with_skew(cx, cy)
                }

                Transform::SkewX(angle) => {
                    let cx = angle.to_radians().tan();

                    Transform2D::with_skew(cx, 0.0)
                }

                Transform::SkewY(angle) => {
                    let cy = angle.to_radians().tan();

                    Transform2D::with_skew(0.0, cy)
                }

                Transform::Matrix(matrix) => {
                    Transform2D::new(matrix.a, matrix.b, matrix.c, matrix.d, matrix.e, matrix.f)
                }
                _ => Transform2D::identity(),
            };

            result.premultiply(&t);
        }

        result
    }

    pub fn identity() -> Self {
        Self([1.0, 0.0, 0.0, 1.0, 0.0, 0.0])
    }

    pub fn rotate(&mut self, a: f32) {
        let cs = a.cos();
        let sn = a.sin();

        self[0] = cs;
        self[1] = sn;
        self[2] = -sn;
        self[3] = cs;
        self[4] = 0.0;
        self[5] = 0.0;
    }

    pub fn get_rotate(&self) -> f32 {
        self[0].acos().to_degrees()
    }

    pub fn inverse(&mut self) {
        let t = *self;
        let det = t[0] as f64 * t[3] as f64 - t[2] as f64 * t[1] as f64;

        if det > -1e-6 && det < 1e-6 {
            *self = Self::identity();
        }

        let invdet = 1.0 / det;

        self[0] = (t[3] as f64 * invdet) as f32;
        self[2] = (-t[2] as f64 * invdet) as f32;
        self[4] = ((t[2] as f64 * t[5] as f64 - t[3] as f64 * t[4] as f64) * invdet) as f32;
        self[1] = (-t[1] as f64 * invdet) as f32;
        self[3] = (t[0] as f64 * invdet) as f32;
        self[5] = ((t[1] as f64 * t[4] as f64 - t[0] as f64 * t[5] as f64) * invdet) as f32;
    }

    pub fn translate(&mut self, tx: f32, ty: f32) {
        self[4] = tx;
        self[5] = ty;
    }

    pub fn scale(&mut self, sx: f32, sy: f32) {
        self[0] = sx;
        self[3] = sy;
    }

    pub fn skew(&mut self, cx: f32, cy: f32) {
        self[1] = cy;
        self[2] = cx;
    }

    pub fn with_translate(tx: f32, ty: f32) -> Self {
        let mut t = Self::identity();
        t.translate(tx, ty);
        t
    }

    pub fn with_rotate(angle: f32) -> Self {
        let mut t = Self::identity();
        t.rotate(angle);
        t
    }

    pub fn with_scale(sx: f32, sy: f32) -> Self {
        let mut t = Self::identity();
        t.scale(sx, sy);
        t
    }

    pub fn with_skew(cx: f32, cy: f32) -> Self {
        let mut t = Self::identity();
        t.skew(cx, cy);
        t
    }

    pub fn transform_point(&self, sx: f32, sy: f32) -> (f32, f32) {
        let dx = sx * self[0] + sy * self[2] + self[4];
        let dy = sx * self[1] + sy * self[3] + self[5];
        (dx, dy)
    }

    pub fn multiply(&mut self, other: &Self) {
        let t0 = self[0] * other[0] + self[1] * other[2];
        let t2 = self[2] * other[0] + self[3] * other[2];
        let t4 = self[4] * other[0] + self[5] * other[2] + other[4];
        self[1] = self[0] * other[1] + self[1] * other[3];
        self[3] = self[2] * other[1] + self[3] * other[3];
        self[5] = self[4] * other[1] + self[5] * other[3] + other[5];
        self[0] = t0;
        self[2] = t2;
        self[4] = t4;
    }

    pub fn premultiply(&mut self, other: &Self) {
        let mut other = *other;
        other.multiply(self);
        *self = other;
    }
}

impl Default for Transform2D {
    fn default() -> Self {
        Self::identity()
    }
}

impl Index<usize> for Transform2D {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Transform2D {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
