mod graphics;

use std::cell::RefCell;
use std::rc::Rc;

use winit::dpi::PhysicalSize;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode,
                   WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::WindowBuilder;

use fission::graphite::{ConvFrom, U, U2};
use fission::renderer::RenderState;
use fission::util::progress::Progress;

use graphics::GPU;

fn main() -> anyhow::Result<()> {
    // Parse Args
    let mut args = std::env::args();
    args.next().unwrap();

    let scene_file = match args.next() {
        Some(arg) => arg,
        None => anyhow::bail!("Usage: fission <scene_description.yaml> [render_progress.state]"),
    };
    let state_file = args.next();

    // Setup Renderer
    let (renderer, running) = fission::load_from_file(&scene_file, state_file)?;

    // Setup Window and GPU Pipeline
    let mut event_loop = EventLoop::with_user_event();
    let el_proxy = event_loop.create_proxy();
    let (window, mut pipeline) = {
        let _p = Progress::indeterminate("Setting up Window and GPU Pipeline");
        let dims = U2::of(renderer.state.img.rect.dims);
        let size: PhysicalSize<u32> = <(U, U)>::from(dims).into();
        let window = WindowBuilder::new()
            .with_title(format!("Fission - Rendering: {}", scene_file))
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_max_inner_size(size)
            .with_resizable(false)
            .build(&event_loop).unwrap();

        let pipeline = futures::executor::block_on(GPU::new(&window));
        (window, pipeline)
    };

    let frame_rx = renderer.render();

    let final_state = Rc::new(RefCell::new(RenderState::default()));
    let final_state_memo = final_state.clone();

    std::thread::spawn(move || {
        for frame in frame_rx {
            let _ = el_proxy.send_event(frame);
        }
    });

    // Event Loop
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
            Event::MainEventsCleared => window.request_redraw(),
            _ => ()
        }
    });
    running.store(false, std::sync::atomic::Ordering::SeqCst);

    fission::save_to_file(scene_file, &Rc::try_unwrap(final_state).unwrap().into_inner())?;
    Ok(())
}
