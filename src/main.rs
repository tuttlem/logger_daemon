mod daemon;
mod signals;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    log::info!("Starting daemonization process...");

    // Daemonize the process
    unsafe { daemon::daemonize()?; }

    // Set up signal handling
    signals::setup_signal_handlers()?;

    // Main loop
    loop {
        log::info!("Daemon is running...");
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
