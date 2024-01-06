fn add_tile_to_tilemap(tilemap: &mut learn_rust::Tilemap, position: learn_rust::Vector2, value: char) {
    match tilemap.add_tile(position, value) {
        Ok(msg) => println!("{msg}"),
        Err(msg) => println!("{msg}")
    }
}

fn main() {
    let mut tilemap = learn_rust::Tilemap::new('-');

    add_tile_to_tilemap(&mut tilemap, learn_rust::Vector2::new(0, 0), 'O');
    add_tile_to_tilemap(&mut tilemap, learn_rust::Vector2::new(3, 0), 'A');
    add_tile_to_tilemap(&mut tilemap, learn_rust::Vector2::new(5, 2), 'X');
    add_tile_to_tilemap(&mut tilemap, learn_rust::Vector2::new(2, 1), 'U');
    add_tile_to_tilemap(&mut tilemap, learn_rust::Vector2::new(3, 5), 'V');
    add_tile_to_tilemap(&mut tilemap, learn_rust::Vector2::new(1, 4), 'H');

    println!("{}", tilemap.build());
}
