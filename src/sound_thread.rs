use crate::key_sounds::KeySounds;
use kira::effect::reverb::ReverbBuilder;
use kira::track::TrackBuilder;
use kira::{AudioManager, AudioManagerSettings, DefaultBackend};
use std::sync::mpsc::Receiver;
use std::thread;

pub(crate) fn sound_thread(rx: Receiver<rdev::Key>) {
    thread::spawn(move || {
        // TODO: Handle errors
        let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
            .expect("FIRST DEATH");

        // TODO: Handle errors
        let mut track = manager
            .add_sub_track({
                let mut builder = TrackBuilder::new();
                builder.add_effect(ReverbBuilder::new().damping(0.05).feedback(0.2));
                builder
            })
            .expect("NO TRAX BUILD");

        let mut key_sounds = KeySounds::new();

        loop {
            match rx.recv() {
                // TODO: Handle errors
                Err(_) => break,
                Ok(key_press) => match key_sounds.sound_for_key(key_press) {
                    Some(sound) => match track.play(sound) {
                        Err(_) => {}
                        Ok(_) => {}
                    },
                    None => {}
                },
            }
        }
    });
}
