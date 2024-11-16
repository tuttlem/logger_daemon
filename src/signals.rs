use signal_hook::iterator::Signals;
use std::thread;

pub fn setup_signal_handlers() -> Result<(), Box<dyn std::error::Error>> {
    // Capture termination and interrupt signals
    let mut signals = Signals::new(&[signal_hook::consts::SIGTERM, signal_hook::consts::SIGINT])?;

    thread::spawn(move || {
        for sig in signals.forever() {
            match sig {
                signal_hook::consts::SIGTERM | signal_hook::consts::SIGINT => {
                    log::info!("Received termination signal. Shutting down...");
                    std::process::exit(0);
                }
                _ => {}
            }
        }
    });

    Ok(())
}
