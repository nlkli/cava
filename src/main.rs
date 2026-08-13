use vello::kurbo::Size;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

use vello::peniko::color::palette;
use vello::util::{RenderContext, RenderSurface};
use vello::wgpu::CurrentSurfaceTexture;
use vello::{AaConfig, Renderer, RendererOptions, Scene, kurbo, wgpu};

use std::sync::Arc;

mod camera;
mod cli;
mod document;

use camera::Camera;
use document::Document;

enum RenderState {
    Suspended(Option<Arc<Window>>),
    Active {
        surface: RenderSurface<'static>,
        window: Arc<Window>,
        renderer: Renderer,
    },
}

struct App {
    init_window_attr: Option<WindowAttributes>,
    rctx: RenderContext,
    rstate: RenderState,
    scene: Scene,
    camera: Camera,
    document: Document,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let RenderState::Suspended(cached_window) = &mut self.rstate else {
            return;
        };

        let attr = self.init_window_attr.take().unwrap_or_default();
        let window = cached_window
            .take()
            .unwrap_or_else(|| Arc::new(event_loop.create_window(attr).unwrap()));

        let size = window.inner_size();
        let surface_future = self.rctx.create_surface(
            window.clone(),
            size.width,
            size.height,
            wgpu::PresentMode::AutoVsync,
        );
        let surface = pollster::block_on(surface_future).expect("Error creating surface");
        let renderer = Renderer::new(
            &self.rctx.devices[surface.dev_id].device,
            RendererOptions::default(),
        )
        .expect("Couldn't create renderer");

        self.rstate = RenderState::Active {
            surface,
            window,
            renderer,
        };
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        if let RenderState::Active { window, .. } = &self.rstate {
            self.rstate = RenderState::Suspended(Some(window.clone()));
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let (surface, window, renderer) = match &mut self.rstate {
            RenderState::Active {
                surface,
                window,
                renderer,
            } if window.id() == window_id => (surface, window, renderer),
            _ => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if size.width != 0 && size.height != 0 {
                    self.rctx.resize_surface(surface, size.width, size.height);
                    self.camera.viewport = Size::new(size.width as f64, size.height as f64);
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                self.scene.reset();

                // add_shapes_to_scene(&mut self.scene);
                self.document.render(&mut self.scene, &self.camera);

                let stroke = kurbo::Stroke::new(6.0);
                let rect = kurbo::Rect::new(10.0, 10.0, 240.0, 240.0);
                self.scene.stroke(
                    &stroke,
                    kurbo::Affine::IDENTITY,
                    palette::css::AQUA,
                    None,
                    &rect,
                );

                let width = surface.config.width;
                let height = surface.config.height;

                let device_handle = &self.rctx.devices[surface.dev_id];

                renderer
                    .render_to_texture(
                        &device_handle.device,
                        &device_handle.queue,
                        &self.scene,
                        &surface.target_view,
                        &vello::RenderParams {
                            base_color: palette::css::BLACK,
                            width,
                            height,
                            antialiasing_method: AaConfig::Msaa16,
                        },
                    )
                    .expect("failed to render to surface");

                let surface_texture = match surface.surface.get_current_texture() {
                    CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
                    CurrentSurfaceTexture::Outdated | CurrentSurfaceTexture::Suboptimal(_) => {
                        self.rctx.configure_surface(surface);
                        window.request_redraw();
                        return;
                    }
                    CurrentSurfaceTexture::Occluded | CurrentSurfaceTexture::Timeout => {
                        window.request_redraw();
                        return;
                    }
                    CurrentSurfaceTexture::Lost => panic!("Surface was lost"),
                    CurrentSurfaceTexture::Validation => {
                        panic!("Validation error getting surface")
                    }
                };

                let mut encoder =
                    device_handle
                        .device
                        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("Surface Blit"),
                        });

                surface.blitter.copy(
                    &device_handle.device,
                    &mut encoder,
                    &surface.target_view,
                    &surface_texture
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default()),
                );

                device_handle.queue.submit([encoder.finish()]);

                surface_texture.present();

                device_handle.device.poll(wgpu::PollType::Poll).unwrap();
            }
            _ => (),
        }
    }
}

fn main() {
    let args = cli::Args::parse();

    let event_loop = EventLoop::new().unwrap();

    let mut app = App {
        init_window_attr: Some(args.window_attributes()),
        rctx: RenderContext::new(),
        rstate: RenderState::Suspended(None),
        scene: Scene::new(),
        camera: Camera::new(Size::new(args.width as f64, args.height as f64)),
        document: Document::new(),
    };

    let el = document::ShapeElement::new(
        document::Geometry::Rect(kurbo::Rect::new(10.0, 10.0, 240.0, 240.0)),
        document::Style::filled(palette::css::RED),
    );
    app.document.add(el);

    let _ = event_loop.run_app(&mut app);
}
