mod graphics;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use winit::dpi::PhysicalSize;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode,
                   WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::WindowBuilder;

use fission::graphite::{conv, ConvFrom, U, U2};
use fission::renderer::RenderState;
use fission::util::progress::Progress;

const MIN_FRAME_UPDATE_MS: u64 = 20;

fn main() -> anyhow::Result<()> {
    // Parse Args
    let mut args = std::env::args();
    args.next().unwrap();

    let scene_file = match args.next() {
        Some(arg) => arg,
        None => anyhow::bail!("Usage: gui <scene_description.yaml> [render_progress.state]"),
    };
    let state_file = args.next();

    // Setup Renderer
    let (renderer, running) = fission::load_from_file(&scene_file, state_file)?;

    // Setup Window and GPU Pipeline
    let mut event_loop = EventLoop::with_user_event();
    let el_proxy = event_loop.create_proxy();
    let (window, mut pipeline) = {
        let _p = Progress::indeterminate("Setting up Window and GPU Pipeline");
        let size: PhysicalSize<U> = conv!(renderer.state.img.rect.dims => U2 => (U, U)).into();
        let window = WindowBuilder::new()
            .with_title(format!("Fission - Rendering: {}", scene_file))
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_max_inner_size(size)
            .with_resizable(false)
            .build(&event_loop).unwrap();

        let pipeline = futures::executor::block_on(graphics::Context::new(&window));
        (window, pipeline)
    };

    let frame_rx = renderer.render();

    let final_state = Rc::new(RefCell::new(RenderState::default()));
    let final_state_memo = final_state.clone();

    std::thread::spawn(move || {
        frame_rx.iter().for_each(|frame| { let _ = el_proxy.send_event(frame); });
    });

    // Event Loop
    let mut last_update = Instant::now();
    event_loop.run_return(move |event, _, cflow| {
        *cflow = ControlFlow::Wait;
        match event {
            Event::UserEvent(render_state) => {
                pipeline.update(&render_state.img);
                *final_state_memo.borrow_mut() = render_state;
            }
            Event::RedrawRequested(_) =>
                if pipeline.render().is_err() { *cflow = ControlFlow::Exit },
            Event::WindowEvent { ref event, window_id }
            if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested |
                    WindowEvent::KeyboardInput { input: KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                    .. }, .. } => *cflow = ControlFlow::Exit,
                    _ => ()
                }
            }
            Event::MainEventsCleared => {
                if last_update.elapsed() > Duration::from_millis(MIN_FRAME_UPDATE_MS) {
                    window.request_redraw();
                    last_update = Instant::now();
                }
            }
            _ => ()
        }
    });
    running.store(false, std::sync::atomic::Ordering::SeqCst);

    fission::save_to_file(scene_file, &Rc::try_unwrap(final_state).unwrap().into_inner())?;
    Ok(())
}
