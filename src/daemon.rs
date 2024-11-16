use nix::sys::stat::{umask, Mode};
use nix::sys::signal::{signal, SigHandler, Signal};
use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::env;
use nix::unistd::{ForkResult, fork};

pub unsafe fn daemonize() -> Result<(), Box<dyn std::error::Error>> {
    // First fork
    match fork()? {
        ForkResult::Parent { .. } => std::process::exit(0),
        ForkResult::Child => {}
    }

    // Create a new session
    nix::unistd::setsid()?;

    // Ignore SIGHUP
    unsafe {
        signal(Signal::SIGHUP, SigHandler::SigIgn)?;
    }

    // Second fork
    match fork()? {
        ForkResult::Parent { .. } => std::process::exit(0),
        ForkResult::Child => {}
    }

    // Set working directory to root
    env::set_current_dir("/")?;

    // Set file mask
    umask(Mode::empty());

    // Close and reopen standard file descriptors
    close_standard_fds();

    Ok(())
}

fn close_standard_fds() {
    // Close STDIN, STDOUT, STDERR
    for fd in 0..3 {
        nix::unistd::close(fd).ok();
    }

    // Reopen file descriptors to /dev/null
    let dev_null = File::open("/dev/null").unwrap();
    nix::unistd::dup2(dev_null.as_raw_fd(), 0).unwrap(); // STDIN
    nix::unistd::dup2(dev_null.as_raw_fd(), 1).unwrap(); // STDOUT
    nix::unistd::dup2(dev_null.as_raw_fd(), 2).unwrap(); // STDERR
}
