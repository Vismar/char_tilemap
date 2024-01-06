// -------------------------------------------------------------------------------------------------
// Definition
// -------------------------------------------------------------------------------------------------

/// # Description
/// Object that describes a tile, point on a 'map' described by position and specific 'value',
/// which describes how it looks on said map.
#[derive(Debug)]
pub struct Tile {
    /// # Description
    /// Position represented as [`crate::tilemap::Vector2`] of the [`Tile`] in 2d space.
    pub position: crate::tilemap::Vector2,
    /// # Description
    /// Visual representation of a [`Tile`].
    pub value: char,
}

// -------------------------------------------------------------------------------------------------
// Implementation of traits
// -------------------------------------------------------------------------------------------------

impl Ord for Tile {
    /// # Description
    /// Compares two [`Tile`]s by comparing their positions.
    ///
    /// # Return
    /// * [`Ordering::Less`] in case when `self.position.y` < `other.position.y`,
    ///   or when `self.position.x` < `other.position.x` if `self.position.y` == `other.position.y`.
    /// * [`Ordering::Greater`] in case when `self.position.y` > `other.position.y`,
    ///   or when `self.position.x` > `other.position.x` if `self.position.y` == `other.position.y`.
    /// * [`Ordering::Equal`] in case when `self.position.y` == `other.position.y` and
    ///   `self.position.x` == `other.position.x`.
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.position.cmp(&other.position);
    }
}

impl PartialOrd for Tile {
    /// # Description
    /// Compares two [`Tile`]s by comparing their positions and putting it in [`Some`].
    ///
    /// # Return
    /// [`Some`] with result of [`Tile::cmp()`].
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(&other));
    }
}

impl PartialEq for Tile {
    /// # Description
    /// Checks if [`Tile`]s are equal by comparison of their positions.
    ///
    /// # Return
    /// `true` if `self.position.y` == `other.position.y` and `self.position.x` == `other.position.x`.
    /// Otherwise - `false`.
    fn eq(&self, other: &Self) -> bool {
        return self.position == other.position;
    }
}

impl Eq for Tile {
}

// -------------------------------------------------------------------------------------------------
// Tests
// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::tilemap::{Tile, Vector2};

    const TEST_VALUE_X_0_0: Tile  = Tile { position: Vector2::ZERO, value: 'X' };
    const TEST_VALUE_X_1_1: Tile  = Tile { position: Vector2::ONE, value: 'X' };
    const TEST_VALUE_W_0_0: Tile  = Tile { position: Vector2::ZERO, value: 'W' };


    #[test]
    fn ord() {
        match TEST_VALUE_X_0_0.cmp(&TEST_VALUE_X_0_0) {
            std::cmp::Ordering::Greater => assert!(false),
            std::cmp::Ordering::Less => assert!(false),
            std::cmp::Ordering::Equal => assert!(true),
        }

        match TEST_VALUE_X_0_0.cmp(&TEST_VALUE_W_0_0) {
            std::cmp::Ordering::Greater => assert!(false),
            std::cmp::Ordering::Less => assert!(false),
            std::cmp::Ordering::Equal => assert!(true),
        }

        match TEST_VALUE_X_0_0.cmp(&TEST_VALUE_X_1_1) {
            std::cmp::Ordering::Greater => assert!(false),
            std::cmp::Ordering::Less => assert!(true),
            std::cmp::Ordering::Equal => assert!(false),
        }

        match TEST_VALUE_X_1_1.cmp(&TEST_VALUE_X_0_0) {
            std::cmp::Ordering::Greater => assert!(true),
            std::cmp::Ordering::Less => assert!(false),
            std::cmp::Ordering::Equal => assert!(false),
        }
    }

    #[test]
    fn partial_ord() {
        match TEST_VALUE_X_0_0.partial_cmp(&TEST_VALUE_X_0_0) {
            Some(value) => match value {
                std::cmp::Ordering::Greater => assert!(false),
                std::cmp::Ordering::Less => assert!(false),
                std::cmp::Ordering::Equal => assert!(true),
            },
            None => assert!(false)
        }

        match TEST_VALUE_X_0_0.partial_cmp(&TEST_VALUE_W_0_0) {
            Some(value) => match value {
                std::cmp::Ordering::Greater => assert!(false),
                std::cmp::Ordering::Less => assert!(false),
                std::cmp::Ordering::Equal => assert!(true),
            },
            None => assert!(false)
        }

        match TEST_VALUE_X_0_0.partial_cmp(&TEST_VALUE_X_1_1) {
            Some(value) => match value {
                std::cmp::Ordering::Greater => assert!(false),
                std::cmp::Ordering::Less => assert!(true),
                std::cmp::Ordering::Equal => assert!(false),
            },
            None => assert!(false)
        }

        match TEST_VALUE_X_1_1.partial_cmp(&TEST_VALUE_X_0_0) {
            Some(value) => match value {
                std::cmp::Ordering::Greater => assert!(true),
                std::cmp::Ordering::Less => assert!(false),
                std::cmp::Ordering::Equal => assert!(false),
            },
            None => assert!(false)
        }
    }

    #[test]
    fn equal() {
        assert!(TEST_VALUE_X_0_0 == TEST_VALUE_W_0_0);
        assert!(TEST_VALUE_X_0_0 != TEST_VALUE_X_1_1);
    }
}
