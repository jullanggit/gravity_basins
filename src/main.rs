use std::{mem::MaybeUninit, sync::Arc};

use encase::{ShaderSize, UniformBuffer};
use shader::{AppState, Graviton, Gravitons, bind_groups::BindGroup0};
use wgpu::{
    Backends, Buffer, BufferDescriptor, BufferUsages, Device, DeviceDescriptor, Features, Instance,
    InstanceDescriptor, Limits, MemoryHints, MultisampleState, PowerPreference, PrimitiveState,
    Queue, RenderPipeline, RequestAdapterOptions, Surface, SurfaceConfiguration, VertexStepMode,
};
use winit::{dpi::PhysicalSize, event_loop::EventLoop, window::Window};

#[allow(dead_code)]
mod shader;

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

fn main() {}
