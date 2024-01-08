fn add_tile_to_tilemap(tilemap: &mut char_tilemap::Tilemap, position: char_tilemap::Vector2, value: char) {
    match tilemap.add_tile(position, value) {
        Ok(_) => (),
        Err(msg) => println!("{msg}")
    }
}

fn main() {
    let mut tilemap = char_tilemap::Tilemap::new('-');

    add_tile_to_tilemap(&mut tilemap, char_tilemap::Vector2::new(0, 0), 'O');
    add_tile_to_tilemap(&mut tilemap, char_tilemap::Vector2::new(3, 0), 'A');
    add_tile_to_tilemap(&mut tilemap, char_tilemap::Vector2::new(5, 2), 'X');
    add_tile_to_tilemap(&mut tilemap, char_tilemap::Vector2::new(2, 1), 'U');
    add_tile_to_tilemap(&mut tilemap, char_tilemap::Vector2::new(3, 5), 'V');
    add_tile_to_tilemap(&mut tilemap, char_tilemap::Vector2::new(1, 4), 'H');

    println!("{}", tilemap.build());
}
