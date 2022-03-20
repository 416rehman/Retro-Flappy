pub struct Coordinate<T> {
    pub(crate) value: T,
}

impl<T> Coordinate<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }

    pub fn set(&mut self, value: T) -> &Self {
        self.value = value;
        self
    }
}
