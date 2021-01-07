fn main() -> anyhow::Result<()> {
    // Parse Args
    let mut args = std::env::args();
    args.next().unwrap();

    let scene_file = match args.next() {
        Some(arg) => arg,
        None => anyhow::bail!("Usage: fission <scene_description.yaml> \
                                              [render_progress.state]"),
    };
    let state_file = args.next();

    // Rendering
    let (renderer, running) = fission::load_from_file(&scene_file, state_file)?;

    ctrlc::set_handler(move || {
        running.store(false, std::sync::atomic::Ordering::SeqCst);
    })?;

    let render_state = renderer.render().iter().last().unwrap();
    fission::save_to_file(scene_file, &render_state)
}
