use rdev::{Event, EventType, Key, grab};
use std::sync::mpsc::channel;

mod key_sounds;
mod sound_thread;
mod stdin_sink;

fn main() {
    let (tx, rx) = channel();

    let callback = move |event: Event| -> Option<Event> {
        let event_type = event.event_type;
        let event_name = event.name.clone().unwrap_or_default(); // binding
        let glyph_str = event_name.as_str();
        match (event_type, glyph_str) {
            (EventType::KeyPress(key_press), glyph) => {
                match key_press {
                    Key::Alt
                    | Key::AltGr
                    | Key::ShiftLeft
                    | Key::ShiftRight
                    | Key::ControlLeft
                    | Key::ControlRight => {
                        // TODO set bool for modifier keys and also detect KeyRelease to unset same bool
                        println!("modifier key: '{:?}'", event_type);
                    }
                    Key::Backspace
                    | Key::CapsLock
                    | Key::Delete
                    | Key::DownArrow
                    | Key::End
                    | Key::Escape
                    | Key::Home
                    | Key::LeftArrow
                    | Key::MetaLeft
                    | Key::MetaRight
                    | Key::PageDown
                    | Key::PageUp
                    | Key::Return
                    | Key::RightArrow
                    | Key::Space
                    | Key::Tab
                    | Key::UpArrow
                    | Key::PrintScreen
                    | Key::ScrollLock
                    | Key::Pause
                    | Key::NumLock
                    | Key::IntlBackslash
                    | Key::Insert
                    | Key::KpReturn
                    | Key::Function
                    | Key::KpDelete => {
                        println!("modifier key: '{:?}'", event_type);
                    }
                    Key::Unknown(_) => todo!(),
                    _ => {
                        println!("glyph: '{glyph}'");
                    }
                }
                let _ = tx.send(key_press.clone());
            }
            (EventType::KeyRelease(_), _) => {}
            (_, _) => {}
        }

        Some(event)
    };

    stdin_sink::stdin_sink();

    sound_thread::sound_thread(rx);

    if let Err(error) = grab(callback) {
        println!("Error: {:?}", error)
    }
}
