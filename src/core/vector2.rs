use crate::core::Coordinate;

pub struct Vector2<T> {
    pub(crate) x: Coordinate<T>,
    pub(crate) y: Coordinate<T>,
}

impl<T: Default> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x: Coordinate::new(x),
            y: Coordinate::new(y),
        }
    }
}

impl<T: Default> Default for Vector2<T> {
    fn default() -> Self {
        Vector2::new(T::default(), T::default())
    }
}
