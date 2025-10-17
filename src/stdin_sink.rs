use std::thread::{self};

pub(crate) fn stdin_sink() {
    thread::spawn(|| {
        loop {
            // println!("reading stdin");
            let _ = read_input();
        }
    });
}

// HiddenInput is copied from the rpassword crate and modified to throw away
// the input.
//
// The modifications may not be perfect and a single byte from the last glyph
// may be sit resident in memory so I guess it isn't entirely secure.
// Also, the thread doesn't loop until the Return key is typed but I don't
// know why.
// When this program is enhanced to not need to run as a CLI program then this
// functionality can probably be thrown away because it exists simeple to
// clean up the output when this program happens to be the foreground app and
// the stdin of this program happens to print its input.
#[cfg(target_family = "unix")]
mod unix {
    use libc::{ECHO, ECHONL, TCSANOW, c_int, tcsetattr, termios};
    use std::io::{self, BufRead, Write};
    use std::mem;
    use std::os::unix::io::AsRawFd;

    struct HiddenInput {
        fd: i32,
        term_orig: termios,
    }

    impl HiddenInput {
        fn new(fd: i32) -> io::Result<HiddenInput> {
            // Make two copies of the terminal settings. The first one will be modified
            // and the second one will act as a backup for when we want to set the
            // terminal back to its original state.
            let mut term = safe_tcgetattr(fd)?;
            let term_orig = safe_tcgetattr(fd)?;

            // Hide the password. This is what makes this function useful.
            term.c_lflag &= !ECHO;

            // But don't hide the NL character when the user hits ENTER.
            term.c_lflag |= ECHONL;

            // Save the settings for now.
            io_result(unsafe { tcsetattr(fd, TCSANOW, &term) })?;

            Ok(HiddenInput { fd, term_orig })
        }
    }

    impl Drop for HiddenInput {
        fn drop(&mut self) {
            // Set the mode back to normal
            unsafe {
                tcsetattr(self.fd, TCSANOW, &self.term_orig);
            }
        }
    }

    /// Turns a C function return into an IO Result
    fn io_result(ret: c_int) -> std::io::Result<()> {
        match ret {
            0 => Ok(()),
            _ => Err(std::io::Error::last_os_error()),
        }
    }

    fn safe_tcgetattr(fd: c_int) -> std::io::Result<termios> {
        let mut term = mem::MaybeUninit::<termios>::uninit();
        io_result(unsafe { ::libc::tcgetattr(fd, term.as_mut_ptr()) })?;
        Ok(unsafe { term.assume_init() })
    }

    /// Reads input from the TTY
    pub fn read_input() -> std::io::Result<()> {
        let tty = std::fs::File::open("/dev/tty")?;
        let fd = tty.as_raw_fd();
        let mut reader = io::BufReader::new(tty);

        read_input_from_fd_with_hidden_input(&mut reader, fd)
    }

    /// Reads input from a given file descriptor
    fn read_input_from_fd_with_hidden_input(
        reader: &mut impl BufRead,
        fd: i32,
    ) -> std::io::Result<()> {
        let hidden_input = HiddenInput::new(fd)?;

        let mut buffer = [0; 1];

        reader.read_exact(&mut buffer)?;
        let _ = std::io::sink().write(&mut buffer);

        std::mem::drop(hidden_input);
        Ok(())
    }
}

#[cfg(target_family = "unix")]
pub use unix::read_input;
