use std::{mem::MaybeUninit, sync::Arc};

use encase::{ShaderSize, UniformBuffer};
use shader::{AppState, Graviton, Gravitons, bind_groups::BindGroup0};
use wgpu::{
    Backends, Buffer, BufferDescriptor, BufferUsages, Color, CommandEncoderDescriptor, Device,
    DeviceDescriptor, Features, Instance, InstanceDescriptor, Limits, LoadOp, MemoryHints,
    MultisampleState, Operations, PowerPreference, PrimitiveState, Queue,
    RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RequestAdapterOptions,
    StoreOp, Surface, SurfaceConfiguration, TextureViewDescriptor, VertexStepMode,
};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

#[allow(dead_code)]
mod shader;

#[derive(Default)]
struct State {
    wgpu: Option<WgpuState>,
    app: AppState,
}

struct WgpuState {
    window: Arc<Window>,
    // No sure why this is fine, but its in the tutorial
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    size: PhysicalSize<u32>,
    config: SurfaceConfiguration,
    pipeline: RenderPipeline,
    bind_group0: BindGroup0,
    uniform_buffer: Buffer,
}
impl WgpuState {
    async fn new(window: Arc<Window>) -> Self {
        // Create a wgpu instance
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });

        // Create a surface for the window
        let surface = instance.create_surface(window.clone()).unwrap();
        // Get adapter (physical device (i think))
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        // Get the logical device and its queue
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: None,
                    required_features: Features::empty(),
                    required_limits: Limits::downlevel_defaults(),
                    memory_hints: MemoryHints::Performance,
                },
                None,
            )
            .await
            .unwrap();

        // Get the size of the window
        let size = window.inner_size();

        // Get the preferred format of the surface
        let caps = surface.get_capabilities(&adapter);
        let surface_format = caps.formats[0];

        // Configure the surface
        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
        surface.configure(&device, &config);

        // Use the generated bindings to create the render pipeline
        let module = shader::create_shader_module(&device);
        let render_pipeline_layout = shader::create_pipeline_layout(&device);
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: shader::vertex_state(&module, &shader::vs_main_entry(VertexStepMode::Vertex)),
            fragment: Some(shader::fragment_state(
                &module,
                &shader::fs_main_entry([Some(surface_format.into())]),
            )),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
            cache: Default::default(),
        });

        // Create the uniform buffer
        let uniform_buffer = device.create_buffer(&BufferDescriptor {
            label: None,
            // TODO: see if this is right
            size: shader::AppState::SHADER_SIZE.into(),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create the bind group 0
        let bind_group0 = shader::bind_groups::BindGroup0::from_bindings(
            &device,
            shader::bind_groups::BindGroupLayout0 {
                app_state: uniform_buffer.as_entire_buffer_binding(),
            },
        );

        Self {
            window,
            surface,
            device,
            queue,
            size,
            config,
            pipeline,
            bind_group0,
            uniform_buffer,
        }
    }
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.window.request_redraw();
        }
    }
}

impl Gravitons {
    /// Uses unitialised memory, any reads beyond the length are undefined behavior
    #[allow(clippy::uninit_assumed_init)]
    #[allow(invalid_value)]
    unsafe fn new() -> Self {
        Self {
            length: 0,
            gravitons: unsafe { MaybeUninit::uninit().assume_init() },
        }
    }
    fn push(&mut self, graviton: Graviton) {
        self.gravitons[self.length as usize] = graviton;
        self.length += 1;
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            position: glam::Vec2::ZERO,
            zoom: 1.,
            drag: 0.02,
            delta_t: 0.05,
            gravitons: unsafe { shader::Gravitons::new() },
        }
    }
}
impl State {
    fn redraw(&mut self) {
        let wgpu_state = self.wgpu.as_mut().unwrap();

        let frame = wgpu_state.surface.get_current_texture().unwrap();
        let view = frame.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = wgpu_state
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Render encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            render_pass.set_pipeline(&wgpu_state.pipeline);

            let mut app_state_bytes = UniformBuffer::new(Vec::<u8>::new());
            app_state_bytes.write(&self.app).unwrap();

            wgpu_state.queue.write_buffer(
                &wgpu_state.uniform_buffer,
                0,
                &app_state_bytes.into_inner(),
            );

            shader::set_bind_groups(&mut render_pass, &wgpu_state.bind_group0);

            render_pass.draw(0..3, 0..1);
        }
        wgpu_state.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}
impl ApplicationHandler for State {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
        self.wgpu = Some(pollster::block_on(WgpuState::new(window)));
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::Resized(new_size) => {
                let wgpu_state = self.wgpu.as_mut().unwrap();
                wgpu_state.resize(new_size);
                wgpu_state.window.request_redraw();
            }
            WindowEvent::CloseRequested => todo!(),
            WindowEvent::Destroyed => todo!(),
            WindowEvent::CursorMoved {
                device_id,
                position,
            } => todo!(),
            WindowEvent::MouseWheel {
                device_id,
                delta,
                phase,
            } => todo!(),
            WindowEvent::MouseInput {
                device_id,
                state,
                button,
            } => todo!(),
            WindowEvent::RedrawRequested => self.redraw(),
            event => {
                dbg!(event);
            }
        }
    }
}

fn run(event_loop: EventLoop<()>) {
    let mut state = State::default();

    event_loop.run_app(&mut state).unwrap();
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    run(event_loop);
}
