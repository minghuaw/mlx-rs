//! Utility types and functions.

/// A convenience trait to convert a single value or a pair of values into a pair of values.
pub trait IntOrPair {
    /// Converts the value into a pair of values.
    fn into_pair(self) -> (i32, i32);
}

impl IntOrPair for i32 {
    fn into_pair(self) -> (i32, i32) {
        (self, self)
    }
}

impl IntOrPair for (i32, i32) {
    fn into_pair(self) -> (i32, i32) {
        self
    }
}

/// A convenience trait to convert a single value or a triple of values into a triple of values.
pub trait IntOrTriple {
    /// Converts the value into a triple of values.
    fn into_triple(self) -> (i32, i32, i32);
}

impl IntOrTriple for i32 {
    fn into_triple(self) -> (i32, i32, i32) {
        (self, self, self)
    }
}

impl IntOrTriple for (i32, i32, i32) {
    fn into_triple(self) -> (i32, i32, i32) {
        self
    }
}
