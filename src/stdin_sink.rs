use std::{
    io::{self, Read as _},
    thread,
};

// TODO: make this work using inspiration from something like rpassword
pub(crate) fn stdin_sink() {
    thread::spawn(|| {
        let mut in_buffer = [32];
        let mut stdin = io::stdin();
        let _ = stdin.read(&mut in_buffer);
    });
}
