// -------------------------------------------------------------------------------------------------
// Definition
// -------------------------------------------------------------------------------------------------

/// # Description
/// Simple object that helps to store unsigned vector, point,
/// or other things that require two unsigned values.
///
/// # Notes
/// [`Vector2::y`] is the first field of the struct to make comparison traits work correctly.
/// By 'correctly' means that when comparing two [`Vector2`] objects,
/// the bigger one is the one with bigger `y` field. If `y` fields are equal,
/// then `x` field should be compared.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct Vector2 {
    /// # Description
    /// `Y` field of the [`Vector2`]. Represents vertical value of a vector.
    pub y: usize,
    /// # Description
    /// `X` field of the [`Vector2`]. Represents horizontal value of a vector.
    pub x: usize,
}

// -------------------------------------------------------------------------------------------------
// Implementation
// -------------------------------------------------------------------------------------------------

impl Vector2 {
    /// # Description
    /// [`Vector2`] with all its fields equal to 0.
    #[allow(dead_code)]
    pub const ZERO: Vector2 = Vector2 { x: 0, y: 0 };
    /// # Description
    /// [`Vector`] with all its fields equal to 1.
    #[allow(dead_code)]
    pub const ONE: Vector2 = Vector2 { x: 1, y: 1 };
    /// # Description
    /// [`Vector2`] with all its fields equal to [`usize::MAX`].
    #[allow(dead_code)]
    pub const MAX: Vector2 = Vector2 { x: usize::MAX, y: usize::MAX };

    /// # Description
    /// Creates new [`Vector2`] using specified values.
    ///
    /// # Arguments
    /// * `x: usize` - x value of a new [`Vector2`].
    /// * `y: usize` - y value of a new [`Vector2`].
    ///
    /// # Return
    /// Newly created [`Vector2`] with specified values.
    pub fn new(x: usize, y: usize) -> Vector2 {
        return Vector2 { x, y }
    }
}

// -------------------------------------------------------------------------------------------------
// Implementation of traits
// -------------------------------------------------------------------------------------------------

impl std::ops::Add<Vector2> for Vector2 {
    type Output = Self;

    /// # Description
    /// Adds value of `rhs` parameter to `self`.
    ///
    /// # Arguments
    /// * `rhs: Vector2` - [`Vector2`] which values will be added to initiator.
    ///
    /// # Return
    /// New [`Vector2`], where:\
    /// `x: self.x + rhs.x`\
    /// `y: self.y + rhs.y`
    ///
    /// # Panic!
    /// Will [`panic!`] in case of overflow.
    fn add(self, rhs: Vector2) -> Self::Output {
        return Vector2 { x: self.x + rhs.x, y: self.y + rhs.y };
    }
}

impl std::ops::Sub<Vector2> for Vector2 {
    type Output = Self;

    /// # Description
    /// Subtracts value of `rhs` parameter from `self`.
    ///
    /// # Arguments
    /// * `rhs: Vector2` - Another [`Vector2`] which values will be subtracted from initiator.
    ///
    /// # Return
    /// New [`Vector2`], where:\
    /// `x: self.x - rhs.x`\
    /// `y: self.y - rhs.y`
    ///
    /// # Panic!
    /// Will [`panic!`] in case of overflow.
    fn sub(self, rhs: Vector2) -> Self::Output {
        return Vector2 { x: self.x - rhs.x, y: self.y - rhs.y };
    }
}

impl std::fmt::Display for Vector2 {
    /// # Description
    /// Writes correctly [`Vector2`] to a formatter.
    ///
    /// # Arguments
    /// * `formatter: &mut std::fmt::Formatter<'_>` - Formatter to which this [`Vector2`] will be put.
    ///
    /// # Return
    /// [`std::fmt::Result`] from [`write!`] macro.
    ///
    /// # Format
    /// Resulting format: `{ x: {self.x}, y: {self.y} }`
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(formatter, "{{ x: {}, y: {} }}", self.x, self.y);
    }
}

// -------------------------------------------------------------------------------------------------
// Tests
// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::tilemap::Vector2;

    const TEST_VALUE: Vector2 = Vector2 { x: 1, y: 5};
    const TEST_VALUE_DOUBLED: Vector2 = Vector2 { x: TEST_VALUE.x * 2, y: TEST_VALUE.y * 2};

    #[test]
    fn new() {
        assert_eq!(Vector2::new(Vector2::ZERO.x, Vector2::ZERO.y), Vector2::ZERO);
        assert_eq!(Vector2::new(TEST_VALUE.x, TEST_VALUE.y), TEST_VALUE);
        assert_eq!(Vector2::new(TEST_VALUE_DOUBLED.x, TEST_VALUE_DOUBLED.y), TEST_VALUE_DOUBLED);
    }

    #[test]
    fn add() {
        assert_eq!(Vector2::ZERO + TEST_VALUE, TEST_VALUE);
        assert_eq!(TEST_VALUE + TEST_VALUE, TEST_VALUE_DOUBLED);
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn add_overflow() {
        let _overflow_value = Vector2::MAX + Vector2::ONE;
    }

    #[test]
    fn sub() {
        assert_eq!(TEST_VALUE - Vector2::ZERO, TEST_VALUE);
        assert_eq!(TEST_VALUE_DOUBLED - TEST_VALUE, TEST_VALUE);
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn sub_overflow() {
        let _overflow_value = Vector2::ZERO - Vector2::ONE;
    }

    #[test]
    fn format() {
        assert_eq!(format!("{}", Vector2::ZERO), "{ x: 0, y: 0 }");
        assert_eq!(format!("{}", Vector2::ONE),  "{ x: 1, y: 1 }");
        assert_eq!(format!("{}", TEST_VALUE),    "{ x: 1, y: 5 }");
    }
}
