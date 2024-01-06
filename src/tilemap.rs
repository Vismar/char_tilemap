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
    empty_tile: char,
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
    /// Adds a new [`Tile`] at the specified position and with specified value.
    ///
    /// # Arguments
    /// * `position: Vector2` - Position of a new [`Tile`].
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

}
