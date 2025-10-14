use anyhow::Result;
use key_sounds::KeySounds;
use kira::{AudioManager, AudioManagerSettings, DefaultBackend};
use rdev::{Event, EventType, grab};
use std::sync::mpsc::channel;
use std::thread;

mod key_sounds;

fn main() -> Result<()> {
    let (tx, rx) = channel();

    let callback = move |event: Event| -> Option<Event> {
        match event.event_type {
            EventType::KeyPress(key_press) => {
                println!("Key pressed: {:?}", key_press);
                let _ = tx.send(key_press.clone());
            }
            _ => {}
        }
        Some(event)
    };

    thread::spawn(move || {
        let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
            .expect("FIRST DEATH");

        let key_sounds = KeySounds::new();

        loop {
            match rx.recv() {
                Err(e) => break,
                Ok(key_press) => match key_sounds.sound_for_key(key_press) {
                    Some(sound) => match manager.play(sound) {
                        Err(_) => {}
                        Ok(_) => {}
                    },
                    None => {}
                },
            }
        }
    });

    if let Err(error) = grab(callback) {
        println!("Error: {:?}", error)
    }

    Ok(())
}
