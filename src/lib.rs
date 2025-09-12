mod state;

use state::RendererState;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use wgpu::COPY_BUFFER_ALIGNMENT;
use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        use winit::dpi::PhysicalSize;
        let _ = window.request_inner_size(PhysicalSize::new(450, 400));

        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-example")?;
                let canvas = web_sys::Element::from(window.canvas()?);
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    let mut state = RendererState::new(&window).await;

    event_loop
        .run(move |event, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() && !state.input(event) => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            state: ElementState::Pressed,
                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                            ..
                        },
                    ..
                } => control_flow.exit(),
                WindowEvent::Resized(new_window_size) => state.resize(*new_window_size),

                WindowEvent::RedrawRequested => {
                    state.update();
                    match state.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                        Err(wgpu::SurfaceError::OutOfMemory) => control_flow.exit(),

                        Err(e) => eprintln!("{:?}", e),
                    }
                }

                _ => {}
            },
            Event::AboutToWait => {
                // RedrawRequested will only trigger once unless we manually
                // request it.
                state.window().request_redraw();
            }
            _ => {}
        })
        .unwrap();
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    const fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

enum VertexState {
    First,
    Second,
}

const FIRST_VERTEX_STATE: [Vertex; 5] = [
    Vertex {
        position: [-0.0868241, 0.49240386, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    Vertex {
        position: [-0.49513406, 0.06958647, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    Vertex {
        position: [-0.21918549, -0.44939706, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    Vertex {
        position: [0.35966998, -0.3473291, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    Vertex {
        position: [0.44147372, 0.2347359, 0.0],
        color: [0.5, 0.0, 0.5],
    },
];
const FIRST_INDEX_STATE: [u16; 9] = [0, 1, 4, 1, 2, 4, 2, 3, 4];

const SECOND_VERTEX_STATE: [Vertex; 5] = [
    Vertex {
        position: [-0.0868241, 0.49240386, 0.0],
        color: [0.2, 0.0, 0.5],
    },
    Vertex {
        position: [-0.39513406, 0.06958647, 0.0],
        color: [0.3, 0.2, 0.5],
    },
    Vertex {
        position: [-0.41918549, -0.44939706, 0.0],
        color: [0.1, 0.9, 0.7],
    },
    Vertex {
        position: [0.0, -0.3473291, 0.0],
        color: [0.3, 0.3, 0.2],
    },
    Vertex {
        position: [0.84147372, 0.5347359, 0.0],
        color: [0.4, 0.4, 0.4],
    },
];
const SECOND_INDEX_STATE: [u16; 9] = [0, 1, 4, 1, 2, 4, 2, 3, 4];

fn calculate_padded_size(unpadded_size: usize, alignment: usize) -> usize {
    (unpadded_size + alignment - 1) / alignment * alignment
}

impl VertexState {
    pub fn vertices(&self) -> &[Vertex] {
        match self {
            VertexState::First => &FIRST_VERTEX_STATE,
            VertexState::Second => &SECOND_VERTEX_STATE,
        }
    }

    pub fn indexes(&self) -> Vec<u16> {
        let index = match self {
            VertexState::First => &FIRST_INDEX_STATE,
            VertexState::Second => &SECOND_INDEX_STATE,
        };

        let mut arr = Vec::from(index);

        let new_len = calculate_padded_size(index.len(), COPY_BUFFER_ALIGNMENT as usize);

        arr.resize(new_len, 0);

        arr
    }

    /// Rotates through the available vertex states
    pub fn next(&self) -> Self {
        match self {
            VertexState::First => VertexState::Second,
            VertexState::Second => VertexState::First,
        }
    }
}
