use {
    anyhow::Result, clap::Parser, flexi_logger::Logger, std::path::PathBuf,
    sts::graphics::Recompiler,
};

#[derive(Parser, Debug, Eq, PartialEq)]
#[command(version, about, long_about=None)]
struct Args {
    /// The path to the shader to watch.
    pub fragment_shader_path: PathBuf,
}

fn main() -> Result<()> {
    Logger::try_with_env_or_str("trace")
        .unwrap()
        .start()
        .expect("Unable to start the logger!");

    let args = Args::parse();

    let mut fragment_compiler = Recompiler::new(&args.fragment_shader_path)?;

    while !fragment_compiler.check_for_update()? {
        std::hint::spin_loop();
    }
    log::info!("Got updated shader source!");

    Ok(())
}
