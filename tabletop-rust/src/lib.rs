pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}


use godot::prelude::*;

struct TabletopExtension;

#[gdextension]
unsafe impl ExtensionLibrary for TabletopExtension {}

mod character;
mod player;
mod identifiable;
mod token;
mod camera;
mod user;
mod tabletop;
mod player_character_portraits;
mod map;

