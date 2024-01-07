pub use common_types::Vector2;
pub use tile::Tile;

mod common_types;
mod tile;

// -------------------------------------------------------------------------------------------------
// Definition
// -------------------------------------------------------------------------------------------------

/// # Description
/// Small enum that helps identify the result of the private [`Tilemap::build_row()`] method.
enum DrawLineState {
    /// # Description
    /// Identifies that after the execution of the [`Tilemap::build_row()`] new line should be started.
    NewLine,
    /// # Description
    /// Identifies that after the execution of the the [`Tilemap::build_row()`] index is till pointing
    /// tp the same line.
    /// # Value
    /// * `usize` - Index at which [`Tilemap::build_row()`] stopped. Build process should be continued
    ///             from this position.
    SameLine(usize)
}

/// # Description
/// Object that describes and stores 2d tilemap. Provides to means to add/delete/modify tiles in it.
/// Also provides method to create string representation of the tilemap.
#[derive(Debug)]
pub struct Tilemap {
    /// # Description
    /// Value of the empty tile that will be used during the build process of the tile map in
    /// [`Tilemap::build()`] method.
    pub empty_tile: char,
    /// # Description
    /// Size of the tilemap. Depends on the positions of tile that are stored within.
    /// Always equals to the furthest coordinates of tiles on X and Y axes.
    size: Vector2,
    /// # Description
    /// Stores all tiles of this tilemap. They are sorted so it would be much easier to build
    /// tilemap to a string representation.
    tiles: sorted_vec::SortedSet<Tile>
}

// -------------------------------------------------------------------------------------------------
// Implementation
// -------------------------------------------------------------------------------------------------

impl Tilemap {
    /// # Description
    /// Creates hew [`Tilemap`] with specified empty tile value.
    ///
    /// # Arguments
    /// * `empty_tile: char` - Value that will be used for empty tiles during the build of the [`Tilemap`].
    ///
    /// # Return
    /// New instance of the [`Tilemap`].
    pub fn new(empty_tile: char) -> Tilemap {
        return Tilemap {
            empty_tile,
            size: Vector2::new(0, 0),
            tiles: sorted_vec::SortedSet::new(),
        }
    }

    /// # Description
    /// Returns size of the [`Tilemap`].
    ///
    /// # Return
    /// [`Vector2`] that stores size of the [`Tilemap`].
    pub fn size(&self) -> Vector2 {
        return self.size;
    }

    /// # Description
    /// Adds a new [`Tile`] at the specified position and with specified value.
    ///
    /// # Arguments
    /// * `position: Vector2` - Position represented as [`Vector2`] of a new [`Tile`].
    /// * `value: char` - Value as [`char`] of a new [`Tile`].
    ///
    /// # Return
    /// * [`Result::Ok`] will be returned in case of successful addition of a new [`Tile`].
    /// * [`Result::Err`] will be returned if tile at the specified position already exists.
    /// All [`String`] values that are returned with [`Result`] contains log message.
    pub fn add_tile(&mut self, position: Vector2, value: char) -> Result<String, String> {
        let new_tile = Tile { position, value };
        if !self.tiles.contains(&new_tile)
        {
            self.tiles.push(new_tile);
            self.size.x = std::cmp::max(self.size.x, position.x + 1);
            self.size.y = std::cmp::max(self.size.y, position.y + 1);
            return Ok(
                format!("New tile was added at {position} with value \'{value}\'. \
                         New tilemap size is {}",
                        self.size));
        }

        return Err(format!("Failed to add new tile at {position} with value \'{value}\'"));
    }

    /// # Description
    /// Removes tile at the specified position if it exists.
    ///
    /// # Arguments
    /// * `position: Vector2` - Position represented as [`Vector2`] at which [`Tile`] should be removed.
    ///
    /// # Return
    /// * [`Result::Ok`] if at the specified position [`Tile`] did exist and was removed.
    /// * [`Result::Err`] if at the specified position [`Tile`] did not exist.
    pub fn remove_tile(&mut self, position: Vector2) -> Result<(), String> {
        if let Some(index) = self.tiles.iter().position(|tile| tile.position == position) {
            self.tiles.remove_index(index);
            return Ok(());
        }

        return Err(format!("There is no tile at the position {position}"));
    }

    /// # Description
    /// Builds [`Tilemap`] into the string representation. X = 0 is a top row, Y = 0 is a left column.
    ///
    /// # Return
    /// A new [`String`] that contains representation of a [`Tilemap`].
    pub fn build(&self) -> String {
        let mut result = String::new();
        let mut x = 0;
        let mut y = 0;

        // Draw all stored tile
        for (_, tile) in self.tiles.iter().enumerate() {
            // If current tile is on different row, draw empty rows until we reach required one
            while y < tile.position.y {
                match self.build_row(x, self.size.x, &mut result) {
                    DrawLineState::SameLine(new_position) => x = new_position,
                    DrawLineState::NewLine => {
                        x = 0;
                        y += 1;
                        result.push('\n');
                    },
                }
            }

            // Draw current line until tile
            match self.build_row(x, tile.position.x, &mut result) {
                DrawLineState::SameLine(new_position) => x = new_position,
                DrawLineState::NewLine => panic!("This branch should not be executed!")
            }

            // Draw tile
            result.push(tile.value);
            x += 1;
        }

        // If all tiles were drawn but we did not reach the end of the row, draw it
        if x < self.size.x {
            self.build_row(x, self.size.x, &mut result);
        }

        return result;
    }

    /// # Description
    /// Builds a row to the specified [`String`] with empty character from `start_position` to
    /// `end_position` using [`Tilemap::empty_tile`].
    ///
    /// # Arguments
    /// * `mut start_position: usize` - Start position of the row.
    ///    It is mutable since it will be used to determine current position of the built row.
    /// * `end_position: usize` - Final position of the row. Tile at this position will not be built.
    /// * `result: &mut String` - [`String`], to which row will be built.
    ///
    /// # Return
    /// * [`DrawLineState::NewLine`] in case if end of the row was reached.
    /// * [`DrawLineState::SameLine`] in case of end of the row was not reached.
    ///   It will contain end position.
    fn build_row(&self, mut start_position: usize, end_position: usize, result: &mut String) -> DrawLineState {
        while start_position < end_position {
            result.push(self.empty_tile);
            start_position += 1;
        }

        return if start_position == self.size.x { DrawLineState::NewLine }
        else { DrawLineState::SameLine(start_position) };
    }
}

// -------------------------------------------------------------------------------------------------
// Tests
// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::{Tilemap, Vector2};

    const EMPTY_TILE_CHAR: char = '-';
    const NUMBER_OF_TILES: usize = 5;
    const TILE_VALUE: char = 'O';

    fn build_test_tilemap(tilemap: &mut Tilemap) {
        for i in 0..NUMBER_OF_TILES {
            match tilemap.add_tile(Vector2::new(i, i), TILE_VALUE) {
                Ok(_) => assert!(true),
                Err(_) => assert!(false)
            }
        }

        assert_eq!(tilemap.size(), Vector2::new(NUMBER_OF_TILES, NUMBER_OF_TILES));
    }

    #[test]
    fn new() {
        let tilemap = Tilemap::new(EMPTY_TILE_CHAR);
        assert_eq!(tilemap.empty_tile, EMPTY_TILE_CHAR);
    }

    #[test]
    fn size() {
        let tilemap = Tilemap::new(EMPTY_TILE_CHAR);
        assert_eq!(tilemap.size(), Vector2::ZERO);
    }

    #[test]
    fn add_tile() {
        let mut tilemap = Tilemap::new(EMPTY_TILE_CHAR);
        match tilemap.add_tile(Vector2::ZERO, 'O') {
            Ok(_) => assert!(true),
            Err(_) => assert!(false)
        }

        assert_eq!(tilemap.size, Vector2::ONE);

        match tilemap.add_tile(Vector2::ZERO, 'O') {
            Ok(_) => assert!(false),
            Err(_) => assert!(true)
        }

        assert_eq!(tilemap.size, Vector2::ONE);
    }

    #[test]
    fn remove_tile() {
        let mut tilemap = Tilemap::new(EMPTY_TILE_CHAR);
        match tilemap.add_tile(Vector2::ZERO, 'O') {
            Ok(_) => assert!(true),
            Err(_) => assert!(false)
        }

        assert_eq!(tilemap.size, Vector2::ONE);

        match tilemap.remove_tile(Vector2::ZERO) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false)
        }

        match tilemap.remove_tile(Vector2::ZERO) {
            Ok(_) => assert!(false),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn build_row() {
        let mut result = String::new();

        let mut tilemap = Tilemap::new(EMPTY_TILE_CHAR);
        build_test_tilemap(&mut tilemap);

        match tilemap.build_row(0, NUMBER_OF_TILES, &mut result) {
            crate::tilemap::DrawLineState::NewLine => assert!(true),
            crate::tilemap::DrawLineState::SameLine(_) => assert!(false)
        }

        let mut ideal_result = String::new();
        ideal_result.push_str(String::from_utf8(vec![EMPTY_TILE_CHAR as u8; NUMBER_OF_TILES]).unwrap().as_str());

        assert_eq!(result, ideal_result);
    }

    #[test]
    fn build() {
        let mut tilemap = Tilemap::new(EMPTY_TILE_CHAR);
        build_test_tilemap(&mut tilemap);

        let mut ideal_result = String::new();
        for i in 0..NUMBER_OF_TILES {
            ideal_result.push_str(String::from_utf8(vec![EMPTY_TILE_CHAR as u8; i]).unwrap().as_str());
            ideal_result.push(TILE_VALUE);
            ideal_result.push_str(String::from_utf8(vec![EMPTY_TILE_CHAR as u8; NUMBER_OF_TILES - i - 1]).unwrap().as_str());
            if i < NUMBER_OF_TILES - 1 {
                ideal_result.push('\n');
            }
        }

        assert_eq!(tilemap.build(), ideal_result);
    }
}
