mod graphics;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use winit::{dpi::PhysicalSize,
            event::{ElementState, Event, KeyboardInput, VirtualKeyCode,
                    WindowEvent},
            event_loop::{ControlFlow, EventLoop},
            platform::run_return::EventLoopExtRunReturn,
            window::WindowBuilder};

use fission::graphite::{ConvFrom, U, U2};
use fission::renderer::RenderState;

use graphics::Pipeline;

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

    // Setup Renderer
    let (renderer, running) = fission::load_from_file(&scene_file, state_file)?;

    // Setup Window
    let dims = U2::of(renderer.state.img.rect.dims);
    let mut event_loop = EventLoop::new();
    let size = PhysicalSize::from(<(U, U)>::from(dims));
    let win = WindowBuilder::new()
        .with_title(format!("Fission - Rendering: {}", scene_file))
        .with_inner_size(size)
        .with_min_inner_size(size)
        .with_max_inner_size(size)
        .with_resizable(false)
        .build(&event_loop).unwrap();

    // Setup GPU
    let mut pipeline = futures::executor::block_on(Pipeline::new(&win, size));

    let frame_rx = renderer.render();

    let final_state = Rc::new(RefCell::new(RenderState::default()));
    let final_state_memo = final_state.clone();

    // Event Loop
    event_loop.run_return(move |event, _, cflow| {
        *cflow = ControlFlow::WaitUntil(Instant::now()
                                        + Duration::from_millis(15));

        if let Ok(state) = frame_rx.try_recv() {
            pipeline.update(&state.img);
            *final_state_memo.borrow_mut() = state;
        }

        match event {
            Event::WindowEvent { ref event, window_id }
            if window_id == win.id() => {
                match event {
                    WindowEvent::CloseRequested |
                    WindowEvent::KeyboardInput { input: KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                    .. }, .. } => *cflow = ControlFlow::Exit,
                    _ => ()
                }
            }
            Event::RedrawRequested(_) =>
                if pipeline.render().is_err() { *cflow = ControlFlow::Exit },
            Event::MainEventsCleared => win.request_redraw(),
            _ => ()
        }
    });
    running.store(false, std::sync::atomic::Ordering::SeqCst);

    fission::save_to_file(scene_file,
                          &Rc::try_unwrap(final_state).unwrap().into_inner())?;
    Ok(())
}
