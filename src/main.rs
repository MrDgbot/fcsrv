use anyhow::Result;
use clap::Parser;
use fcsrv::{daemon, update, Commands, Opt};

fn main() -> crate::Result<()> {
    let opt = Opt::parse();

    match opt.commands {
        Commands::Run(args) => daemon::run(args)?,
        #[cfg(target_family = "unix")]
        Commands::Start(args) => daemon::start(args)?,
        #[cfg(target_family = "unix")]
        Commands::Restart(args) => daemon::restart(args)?,
        #[cfg(target_family = "unix")]
        Commands::Stop => daemon::stop()?,
        #[cfg(target_family = "unix")]
        Commands::Status => daemon::status(),
        #[cfg(target_family = "unix")]
        Commands::Log => daemon::log()?,
        Commands::Update => update::update()?,
    };

    Ok(())
}
