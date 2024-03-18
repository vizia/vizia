use skia_safe::Matrix;
// use femtovg::Matrix;
use vizia_style::{Angle, Scale, Transform, Translate};

use crate::layout::BoundingBox;

/// Trait for converting a transform definition into a `Matrix`.
pub(crate) trait IntoTransform {
    fn as_transform(&self, bounds: BoundingBox, scale_factor: f32) -> Matrix;
}

impl IntoTransform for Translate {
    fn as_transform(&self, bounds: BoundingBox, scale_factor: f32) -> Matrix {
        let tx = self.x.to_pixels(bounds.w, scale_factor);
        let ty = self.y.to_pixels(bounds.h, scale_factor);

        Matrix::translate((tx, ty))
    }
}

impl IntoTransform for Scale {
    fn as_transform(&self, _bounds: BoundingBox, _scale_factor: f32) -> Matrix {
        let sx = self.x.to_factor();
        let sy = self.y.to_factor();

        Matrix::scale((sx, sy))
    }
}

impl IntoTransform for Angle {
    fn as_transform(&self, _bounds: BoundingBox, _scale_factor: f32) -> Matrix {
        let r = self.to_radians();

        Matrix::rotate_rad(r)
    }
}

impl IntoTransform for Vec<Transform> {
    fn as_transform(&self, bounds: BoundingBox, scale_factor: f32) -> Matrix {
        let mut result = Matrix::new_identity();
        for transform in self.iter() {
            let t = match transform {
                Transform::Translate(translate) => {
                    let tx = translate.0.to_pixels(bounds.w, scale_factor);
                    let ty = translate.1.to_pixels(bounds.h, scale_factor);

                    Matrix::translate((tx, ty))
                }

                Transform::TranslateX(x) => {
                    let tx = x.to_pixels(bounds.w, scale_factor);

                    Matrix::translate((tx, 0.0))
                }

                Transform::TranslateY(y) => {
                    let ty = y.to_pixels(bounds.h, scale_factor);

                    Matrix::translate((0.0, ty))
                }

                Transform::Scale(scale) => {
                    let sx = scale.0.to_factor();
                    let sy = scale.1.to_factor();

                    Matrix::scale((sx, sy))
                }

                Transform::ScaleX(x) => {
                    let sx = x.to_factor();

                    Matrix::scale((sx, 1.0))
                }

                Transform::ScaleY(y) => {
                    let sy = y.to_factor();

                    Matrix::scale((1.0, sy))
                }

                Transform::Rotate(angle) => Matrix::rotate_rad(angle.to_radians()),

                Transform::Skew(x, y) => {
                    let cx = x.to_radians().tan();
                    let cy = y.to_radians().tan();

                    Matrix::skew((cx, cy))
                }

                Transform::SkewX(angle) => {
                    let cx = angle.to_radians().tan();

                    Matrix::skew((cx, 0.0))
                }

                Transform::SkewY(angle) => {
                    let cy = angle.to_radians().tan();

                    Matrix::skew((0.0, cy))
                }

                Transform::Matrix(matrix) => {
                    // t = t.new(matrix.a, matrix.b, matrix.c, matrix.d, matrix.e, matrix.f);
                    Matrix::new_identity()
                }
            };

            result = t * result;
        }

        result
    }
}
