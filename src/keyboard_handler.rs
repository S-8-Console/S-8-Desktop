use sdl2::keyboard::{Keycode, Scancode};


pub fn key_to_button(keycode: Keycode) -> Option<usize>
{
    // convert the key from any layout to the ones corresponding to qwerty
    let keycode_converted = Scancode::from_keycode(keycode).unwrap().clone();

    match keycode_converted.to_string().as_str() {
        /* AWSD */
        "A" => Some(0x0),
        "W" => Some(0x1),
        "S" => Some(0x2),
        "D" => Some(0x3),

        /* Arrows */
        "Left" => Some(0x0),
        "Up" => Some(0x1),
        "Down" => Some(0x2),
        "Right" => Some(0x3),

        /* J and K */
        "J" => Some(0x4),
        "K" => Some(0x5),

        _ => None,
    }
}