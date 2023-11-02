use femtovg::Transform2D;
use vizia_style::{Angle, Scale, Transform, Translate};

use crate::layout::BoundingBox;

/// Trait for converting a transform definition into a `Transform2D`.
pub(crate) trait IntoTransform {
    fn as_transform(&self, bounds: BoundingBox, scale_factor: f32) -> Transform2D;
}

impl IntoTransform for Translate {
    fn as_transform(&self, bounds: BoundingBox, scale_factor: f32) -> Transform2D {
        let mut result = Transform2D::identity();
        let tx = self.x.to_pixels(bounds.w, scale_factor);
        let ty = self.y.to_pixels(bounds.h, scale_factor);

        result.translate(tx, ty);

        result
    }
}

impl IntoTransform for Scale {
    fn as_transform(&self, _bounds: BoundingBox, _scale_factor: f32) -> Transform2D {
        let mut result = Transform2D::identity();
        let sx = self.x.to_factor();
        let sy = self.y.to_factor();
        result.scale(sx, sy);

        result
    }
}

impl IntoTransform for Angle {
    fn as_transform(&self, _bounds: BoundingBox, _scale_factor: f32) -> Transform2D {
        let mut result = Transform2D::identity();
        let r = self.to_radians();
        result.rotate(r);

        result
    }
}

impl IntoTransform for Vec<Transform> {
    fn as_transform(&self, bounds: BoundingBox, scale_factor: f32) -> Transform2D {
        let mut result = Transform2D::identity();
        for transform in self.iter() {
            let mut t = Transform2D::identity();
            match transform {
                Transform::Translate(translate) => {
                    let tx = translate.0.to_pixels(bounds.w, scale_factor);
                    let ty = translate.1.to_pixels(bounds.h, scale_factor);

                    t.translate(tx, ty);
                }

                Transform::TranslateX(x) => {
                    let tx = x.to_pixels(bounds.w, scale_factor);

                    t.translate(tx, 0.0)
                }

                Transform::TranslateY(y) => {
                    let ty = y.to_pixels(bounds.h, scale_factor);

                    t.translate(0.0, ty)
                }

                Transform::Scale(scale) => {
                    let sx = scale.0.to_factor();
                    let sy = scale.1.to_factor();

                    t.scale(sx, sy)
                }

                Transform::ScaleX(x) => {
                    let sx = x.to_factor();

                    t.scale(sx, 1.0)
                }

                Transform::ScaleY(y) => {
                    let sy = y.to_factor();

                    t.scale(1.0, sy)
                }

                Transform::Rotate(angle) => t.rotate(angle.to_radians()),

                Transform::Skew(x, y) => {
                    let cx = x.to_radians().tan();
                    let cy = y.to_radians().tan();

                    t = t.new(1.0, cx, cy, 1.0, 0.0, 0.0);
                }

                Transform::SkewX(angle) => {
                    let cx = angle.to_radians().tan();

                    t.skew_x(cx)
                }

                Transform::SkewY(angle) => {
                    let cy = angle.to_radians().tan();

                    t.skew_y(cy)
                }

                Transform::Matrix(matrix) => {
                    t = t.new(matrix.a, matrix.b, matrix.c, matrix.d, matrix.e, matrix.f);
                }
            };

            result.premultiply(&t);
        }

        result
    }
}
