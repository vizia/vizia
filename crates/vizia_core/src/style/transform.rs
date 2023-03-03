use vizia_style::Transform;

use crate::cache::BoundingBox;
use femtovg::Transform2D;

pub(crate) trait IntoTransform {
    fn into_transform(&self, parent_bounds: BoundingBox, scale_factor: f32) -> Transform2D;
}

impl IntoTransform for Vec<Transform> {
    fn into_transform(&self, parent_bounds: BoundingBox, scale_factor: f32) -> Transform2D {
        let mut result = Transform2D::identity();
        for transform in self.iter() {
            let mut t = Transform2D::identity();
            match transform {
                Transform::Translate(translate) => {
                    let tx = translate.x.to_pixels(parent_bounds.w) * scale_factor;
                    let ty = translate.y.to_pixels(parent_bounds.h) * scale_factor;

                    t.translate(tx, ty);
                }

                Transform::TranslateX(x) => {
                    let tx = x.to_pixels(parent_bounds.h) * scale_factor;

                    t.translate(tx, 0.0)
                }

                Transform::TranslateY(y) => {
                    let ty = y.to_pixels(parent_bounds.h) * scale_factor;

                    t.translate(0.0, ty)
                }

                Transform::Scale(scale) => {
                    let sx = scale.x.to_factor();
                    let sy = scale.y.to_factor();

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

                    t.skew_x(cx);
                    t.skew_y(cy);
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
