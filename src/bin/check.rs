fn check() -> anyhow::Result<()> {
    // Parse Args
    let mut args = std::env::args();
    args.next().unwrap();

    let scene_file = match args.next() {
        Some(arg) => arg,
        None => anyhow::bail!("Usage: check <scene_description.yaml>"),
    };

    // Load Scene
    fission::load_from_file::<_, &str>(scene_file, None)?;
    Ok(())
}

fn main() {
    std::process::exit(match check() {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("{}", e);
            1
        }
    })
}
