// use crate::Interpolator;

use std::ops::{Index, IndexMut};

use crate::{Context, Entity, Tree};

/// A 2D transform matrix.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Transform2D(pub [f32; 6]);

impl Transform2D {
    pub fn new(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) -> Self {
        Self([a, b, c, d, e, f])
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

pub fn apply_transform(cx: &mut Context, tree: &Tree) {
    //println!("Apply Transform");
    for entity in tree.into_iter() {
        //println!("Entity: {}", entity);

        if entity == Entity::root() {
            continue;
        }

        let parent = tree.get_parent(entity).unwrap();
        //let parent_origin = state.data.get_origin(parent);
        let parent_transform = cx.cache.get_transform(parent);

        cx.cache.set_transform(entity, Transform2D::identity());

        cx.cache.set_transform(entity, parent_transform);

        let bounds = cx.cache.get_bounds(entity);

        //state.data.set_origin(entity, parent_origin);

        if let Some(translate) = cx.style.translate.get(entity) {
            cx.cache.set_translate(entity, *translate);
        }

        if let Some(rotate) = cx.style.rotate.get(entity) {
            let x = bounds.x + (bounds.w / 2.0);
            let y = bounds.y + (bounds.h / 2.0);
            cx.cache.set_translate(entity, (x, y));
            cx.cache.set_rotate(entity, (*rotate).to_radians());
            cx.cache.set_translate(entity, (-x, -y));
        }
        //println!("End");

        if let Some((scalex, scaley)) = cx.style.scale.get(entity) {
            let x = bounds.x + (bounds.w / 2.0);
            let y = bounds.y + (bounds.h / 2.0);
            cx.cache.set_translate(entity, (x, y));
            cx.cache.set_scale(entity, (*scalex, *scaley));
            cx.cache.set_translate(entity, (-x, -y));
        }
    }
}
