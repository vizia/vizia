use femtovg::Transform2D;

/// Respresents the axis-aligned bounding box of a view.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoundingBox {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl std::fmt::Display for BoundingBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ x: {:?}, y: {:?}, w: {:?}, h:{:?} }}", self.x, self.y, self.w, self.h)
    }
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, w: std::f32::MAX, h: std::f32::MAX }
    }
}

impl BoundingBox {
    /// Construct a [`BoundingBox`] from checked minimum and maximum values.
    #[inline(always)]
    pub fn from_min_max(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> BoundingBox {
        BoundingBox { x: min_x, y: min_y, w: max_x - min_x, h: max_y - min_y }
    }

    /// Left side of bounds equivalent to `x`.
    #[inline(always)]
    pub fn left(&self) -> f32 {
        self.x
    }

    /// Top of bounds equivalent to `y`.
    #[inline(always)]
    pub fn top(&self) -> f32 {
        self.y
    }

    /// Bounds width equivalent to `w`.
    #[inline(always)]
    pub fn width(&self) -> f32 {
        self.w
    }

    /// Bounds height equivalent to `h`.
    #[inline(always)]
    pub fn height(&self) -> f32 {
        self.h
    }

    /// Right side of bounds.
    #[inline(always)]
    pub fn right(&self) -> f32 {
        self.left() + self.width()
    }

    /// Bottom side of bounds.
    #[inline(always)]
    pub fn bottom(&self) -> f32 {
        self.top() + self.height()
    }

    /// Horizontal and vertical center of bounds.
    #[inline(always)]
    pub fn center(&self) -> (f32, f32) {
        ((self.width() / 2f32) + self.x, (self.height() / 2f32) + self.y)
    }

    /// Left center of bounds.
    #[inline(always)]
    pub fn center_left(&self) -> (f32, f32) {
        (self.left(), (self.height() / 2f32) + self.top())
    }

    /// Right center of bounds.
    #[inline(always)]
    pub fn center_right(&self) -> (f32, f32) {
        (self.right(), (self.height() / 2f32) + self.top())
    }

    /// Top center of bounds.
    #[inline(always)]
    pub fn center_top(&self) -> (f32, f32) {
        ((self.width() / 2f32) + self.left(), self.top())
    }

    /// Bottom center of bounds.
    #[inline(always)]
    pub fn center_bottom(&self) -> (f32, f32) {
        ((self.width() / 2f32) + self.left(), self.bottom())
    }

    /// Bottom left point of bounds.
    #[inline(always)]
    pub fn bottom_left(&self) -> (f32, f32) {
        (self.bottom(), self.left())
    }

    /// Bottom right point of bounds.
    #[inline(always)]
    pub fn bottom_right(&self) -> (f32, f32) {
        (self.bottom(), self.right())
    }

    /// Top left point of bounds.
    #[inline(always)]
    pub fn top_left(&self) -> (f32, f32) {
        (self.top(), self.left())
    }

    /// Top right point of bounds.
    #[inline(always)]
    pub fn top_right(&self) -> (f32, f32) {
        (self.top(), self.right())
    }

    /// Shrinks by some `amount` in both directions and returns a new [`BoundingBox`].
    #[inline(always)]
    #[must_use]
    pub fn shrink(&self, amount: f32) -> BoundingBox {
        BoundingBox::from_min_max(
            self.left() + amount,
            self.top() + amount,
            self.right() - amount,
            self.bottom() - amount,
        )
    }

    /// Shrinks by some `amount` horizontally and returns a new [`BoundingBox`].
    #[inline(always)]
    #[must_use]
    pub fn shrink_horizontal(&self, amount: f32) -> BoundingBox {
        BoundingBox::from_min_max(
            self.left() + amount,
            self.top(),
            self.right() - amount,
            self.bottom(),
        )
    }

    /// Shrinks by some `amount` vertically and returns a new [`BoundingBox`].
    #[inline(always)]
    #[must_use]
    pub fn shrink_vertical(&self, amount: f32) -> BoundingBox {
        BoundingBox::from_min_max(
            self.left(),
            self.top() + amount,
            self.right(),
            self.bottom() - amount,
        )
    }

    /// Shrinks each side by the given separate amounts and returns a new [`BoundingBox`].
    pub fn shrink_sides(&self, left: f32, top: f32, right: f32, bottom: f32) -> BoundingBox {
        BoundingBox::from_min_max(
            self.left() + left,
            self.top() + top,
            self.right() - right,
            self.bottom() - bottom,
        )
    }

    /// Expands by some `amount` in both directions and returns a new [`BoundingBox`].
    #[inline(always)]
    #[must_use]
    pub fn expand(&self, amount: f32) -> BoundingBox {
        BoundingBox::from_min_max(
            self.left() - amount,
            self.top() - amount,
            self.right() + amount,
            self.bottom() + amount,
        )
    }

    /// Expands by some `amount` horizontally and returns a new [`BoundingBox`].
    #[inline(always)]
    #[must_use]
    pub fn expand_horizontal(&self, amount: f32) -> BoundingBox {
        BoundingBox::from_min_max(
            self.left() - amount,
            self.top(),
            self.right() + amount,
            self.bottom(),
        )
    }

    /// Expands by some `amount` vertically and returns a new [`BoundingBox`].
    #[inline(always)]
    #[must_use]
    pub fn expand_vertical(&self, amount: f32) -> BoundingBox {
        BoundingBox::from_min_max(
            self.left(),
            self.top() - amount,
            self.right(),
            self.bottom() + amount,
        )
    }

    pub fn intersection(&self, other: &Self) -> Self {
        let left = self.left().max(other.left());
        let right = self.right().min(other.right());
        let top = self.top().max(other.top());
        let bottom = self.bottom().min(other.bottom());
        BoundingBox::from_min_max(left, top, right, bottom)
    }

    pub fn intersects(&self, other: &Self) -> bool {
        let x_hit = (self.x >= other.x && self.x < other.x + other.w)
            || (other.x >= self.x && other.x < self.x + self.w);
        let y_hit = (self.y >= other.y && self.y < other.y + other.h)
            || (other.y >= self.y && other.y < self.y + self.h);
        x_hit && y_hit
    }

    pub fn contains(&self, other: &Self) -> bool {
        let x_hit = other.x >= self.x && other.x < self.x + self.w;
        let y_hit = other.y >= self.y && other.y < self.y + self.h;
        x_hit && y_hit
    }

    pub fn transform(&self, transform: &Transform2D) -> Self {
        let (tl, tt) = transform.transform_point(self.x, self.y);
        let (tr, tb) = transform.transform_point(self.right(), self.bottom());
        BoundingBox::from_min_max(tl, tt, tr, tb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn rect() -> BoundingBox {
        BoundingBox { x: 100f32, y: 100f32, w: 100f32, h: 100f32 }
    }

    #[test]
    fn get_center() {
        let rect = rect();
        assert_eq!(rect.center(), (150f32, 150f32));
    }

    #[test]
    fn get_center_top() {
        let rect = rect();
        assert_eq!(rect.center_top(), (150f32, 100f32));
    }

    #[test]
    fn get_center_bottom() {
        let rect = rect();
        assert_eq!(rect.center_bottom(), (150f32, 200f32));
    }

    #[test]
    fn get_center_left() {
        let rect = rect();
        assert_eq!(rect.center_left(), (100f32, 150f32));
    }

    #[test]
    fn get_center_right() {
        let rect = rect();
        assert_eq!(rect.center_right(), (200f32, 150f32));
    }

    #[test]
    fn get_left() {
        let rect = rect();
        assert_eq!(rect.left(), 100f32);
    }

    #[test]
    fn get_right() {
        let rect = rect();
        assert_eq!(rect.right(), 200f32);
    }

    #[test]
    fn get_top() {
        let rect = rect();
        assert_eq!(rect.top(), 100f32);
    }

    #[test]
    fn get_bottom() {
        let rect = rect();
        assert_eq!(rect.bottom(), 200f32);
    }

    #[test]
    fn get_shrunken() {
        let rect = rect();
        let a = rect.shrink(25f32);
        let b = BoundingBox { x: 125f32, y: 125f32, w: 50f32, h: 50f32 };
        assert_eq!(a, b);
    }

    #[test]
    fn get_shrunken_horizontal() {
        let rect = rect();
        let a = rect.shrink_horizontal(25f32);
        let b = BoundingBox { x: 125f32, y: 100f32, w: 50f32, h: 100f32 };
        assert_eq!(a, b);
    }

    #[test]
    fn get_shrunken_vertical() {
        let rect = rect();
        let a = rect.shrink_vertical(25f32);
        let b = BoundingBox { x: 100f32, y: 125f32, w: 100f32, h: 50f32 };
        assert_eq!(a, b);
    }

    #[test]
    fn get_expanded() {
        let rect = rect();
        let a = rect.expand(25f32);
        let b = BoundingBox { x: 75f32, y: 75f32, w: 150f32, h: 150f32 };
        assert_eq!(a, b);
    }

    #[test]
    fn get_expanded_horizontal() {
        let rect = rect();
        let a = rect.expand_horizontal(25f32);
        let b = BoundingBox { x: 75f32, y: 100f32, w: 150f32, h: 100f32 };
        assert_eq!(a, b);
    }

    #[test]
    fn get_expanded_vertical() {
        let rect = rect();
        let a = rect.expand_vertical(25f32);
        let b = BoundingBox { x: 100f32, y: 75f32, w: 100f32, h: 150f32 };
        assert_eq!(a, b);
    }
}
