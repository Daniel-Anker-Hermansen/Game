use std::iter::once;

use plugin_lib::ItemGame;
use wgpu_glyph::{ab_glyph, GlyphBrushBuilder, GlyphBrush, Section, Text};
use winit::{window::Window, dpi::PhysicalSize};
use wgpu::{*, util::StagingBelt};

pub struct State {
    window: Window,
    size: PhysicalSize<u32>,
    surface: Surface,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    glyph_brush: GlyphBrush<()>,
    staging_belt: StagingBelt,
    panic: Option<String>,
    item_ids: Vec<(Box<dyn ItemGame + 'static + Send + Sync>, i64)>,
}

impl State {
    pub fn new(window: Window) -> State {
        let size = window.inner_size();
        let instance = Instance::default();
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        let adapter = pollster::block_on(instance.request_adapter(&default())).unwrap();
        let (device, queue) = pollster::block_on(adapter.request_device(&default(), None)).unwrap();
        let font = ab_glyph::FontArc::try_from_slice(include_bytes!("Arialn.ttf")).unwrap();
        let render_format = TextureFormat::Bgra8UnormSrgb;
        let glyph_brush = GlyphBrushBuilder::using_font(font).build(&device, render_format);
        let staging_belt = StagingBelt::new(1024);
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: render_format,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::AutoVsync,
            alpha_mode: CompositeAlphaMode::Auto,
            view_formats: Vec::new(),
        };
        surface.configure(&device, &config);
        State { 
            window,
            size,
            surface,
            device,
            queue,
            config,
            glyph_brush,
            staging_belt,
            panic: None,
            item_ids: Vec::new(),
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&default());
        let mut encoder = self.device.create_command_encoder(&default());
        let load = if self.panic.is_none() { LoadOp::Clear(Color::BLACK) } else { LoadOp::Clear(Color::BLUE) };
        let ops = Operations {
            load,
            ..default()
        };
        let color_attachment = RenderPassColorAttachment {
            view: &view,
            ops,
            resolve_target: None, 
        };
        let render_descriptor = RenderPassDescriptor {
            color_attachments: &[Some(color_attachment)],
            ..default()
        };

        if let Some(msg) = &self.panic {
            let font = ab_glyph::FontArc::try_from_slice(include_bytes!("console.ttf")).unwrap();
            let id = self.glyph_brush.add_font(font);
            self.glyph_brush.queue(Section {
                screen_position: (15.0, 15.0),
                bounds: (self.size.width as f32 - 15.0, self.size.height as f32 - 15.0),
                text: vec![Text::new(&msg)
                    .with_color([1.0, 1.0, 1.0, 1.0])
                    .with_scale(12.0)
                    .with_font_id(id)],
                ..default()
            });
        }
        else {
            let mut height = 30.0;
            for (item, id) in &self.item_ids {
                let msg = format!("{}: {id}", item.name());
                self.glyph_brush.queue(Section {
                    screen_position: (30.0, height),
                    bounds: (self.size.width as f32, self.size.height as f32),
                    text: vec![Text::new(&msg)
                        .with_color([1.0, 1.0, 1.0, 1.0])
                        .with_scale(40.0)],
                    ..default()
                });
                height += 60.0;
            }
        }

        let render_pass = encoder.begin_render_pass(&render_descriptor);
        drop(render_pass);
        self.glyph_brush.draw_queued(&self.device, &mut self.staging_belt, &mut encoder, &view, self.size.width, self.size.height).unwrap();

        self.staging_belt.finish();
        self.queue.submit(once(encoder.finish()));
        output.present();
        self.staging_belt.recall();
        Ok(())
    }

    pub fn panic(&mut self, msg: String) {
        self.panic = Some(msg);
        self.render().unwrap();
    }

    pub fn add_item(&mut self, mut item: impl ItemGame + 'static + Send + Sync) {
        let id = item.id();
        self.item_ids.push((Box::new(item), id)); 
    }

    pub fn iterate(&mut self) {
        if self.panic.is_some() { return; }
        let f = move || self.item_ids.iter_mut()
            .for_each(|(item, id)| {
                *id = item.id();
            });
        let _ = unsafe { std::thread::Builder::new()
            .spawn_unchecked(f) }.unwrap().join();
    }
}

// The free default function since the one in libstd is unstable ...
fn default<T: Default>() -> T {
    Default::default()
}
