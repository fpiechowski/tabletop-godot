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

use godot::prelude::*;
use godot::engine::ISprite2D;

mod character;
mod player;
mod identifiable;
mod token;

