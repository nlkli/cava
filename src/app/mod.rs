use anyhow::Result;
use std::sync::Arc;

use winit::event as WE;
use winit::event_loop as WEL;
// use winit::keyboard as WK;
use winit::window as WW;

use vello as V;
use vello::kurbo as K;

use crate::camera::Camera;
use crate::cli;
use crate::config::Config;
use crate::elem::{Elem, Style as EStyle};

enum RenderState {
    Suspended(Option<Arc<WW::Window>>),
    Active {
        surface: V::util::RenderSurface<'static>,
        window: Arc<WW::Window>,
        renderer: V::Renderer,
        antialiasing_method: V::AaConfig,
    },
}

struct App {
    rctx: V::util::RenderContext,
    rstate: RenderState,
    scene: V::Scene,
    camera: Camera,
    elements: Vec<Elem>,
    config: Config,
}

impl winit::application::ApplicationHandler for App {
    fn resumed(&mut self, el: &WEL::ActiveEventLoop) {
        let RenderState::Suspended(cached_window) = &mut self.rstate else {
            return;
        };

        let attr = self.config.window.attributes();
        let window = cached_window
            .take()
            .unwrap_or_else(|| Arc::new(el.create_window(attr).unwrap()));

        let size = window.inner_size();
        let surface_future = self.rctx.create_surface(
            window.clone(),
            size.width,
            size.height,
            self.config.render.present_mode(),
        );

        let surface = pollster::block_on(surface_future).expect("Error creating surface");

        let renderer = V::Renderer::new(
            &self.rctx.devices[surface.dev_id].device,
            self.config.render.renderer_options(),
        )
        .expect("Couldn't create renderer");

        self.rstate = RenderState::Active {
            surface,
            window,
            renderer,
            antialiasing_method: self.config.render.antialiasing_method(),
        };
    }

    fn window_event(
        &mut self,
        el: &WEL::ActiveEventLoop,
        window_id: WW::WindowId,
        event: WE::WindowEvent,
    ) {
        let (surface, window, renderer, antialiasing_method) = match &mut self.rstate {
            RenderState::Active {
                surface,
                window,
                renderer,
                antialiasing_method,
            } if window.id() == window_id => (surface, window, renderer, antialiasing_method),
            _ => return,
        };

        match event {
            WE::WindowEvent::CloseRequested => el.exit(),
            WE::WindowEvent::RedrawRequested => {
                self.scene.reset();

                self.camera.render(&mut self.scene, &mut self.elements);

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
                            base_color: V::peniko::color::palette::css::BLACK,
                            width,
                            height,
                            antialiasing_method: *antialiasing_method,
                        },
                    )
                    .expect("failed to render to surface");

                let surface_texture = match surface.surface.get_current_texture() {
                    V::wgpu::CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
                    V::wgpu::CurrentSurfaceTexture::Outdated
                    | V::wgpu::CurrentSurfaceTexture::Suboptimal(_) => {
                        self.rctx.configure_surface(surface);
                        window.request_redraw();
                        return;
                    }
                    V::wgpu::CurrentSurfaceTexture::Occluded
                    | V::wgpu::CurrentSurfaceTexture::Timeout => {
                        window.request_redraw();
                        return;
                    }
                    V::wgpu::CurrentSurfaceTexture::Lost => panic!("Surface was lost"),
                    V::wgpu::CurrentSurfaceTexture::Validation => {
                        panic!("Validation error getting surface")
                    }
                };

                let mut encoder = device_handle
                    .device
                    .create_command_encoder(&V::wgpu::CommandEncoderDescriptor::default());

                surface.blitter.copy(
                    &device_handle.device,
                    &mut encoder,
                    &surface.target_view,
                    &surface_texture
                        .texture
                        .create_view(&V::wgpu::TextureViewDescriptor::default()),
                );

                device_handle.queue.submit([encoder.finish()]);

                surface_texture.present();
            }
            _ => {}
        }
    }

    fn suspended(&mut self, _el: &WEL::ActiveEventLoop) {
        if let RenderState::Active { window, .. } = &self.rstate {
            self.rstate = RenderState::Suspended(Some(window.clone()));
        }
    }

    // fn about_to_wait(&mut self, _event_loop: &WEL::ActiveEventLoop) {}

    // fn user_event(&mut self, _event_loop: &WEL::ActiveEventLoop, event: UserEvent) {}
}

pub fn run() -> Result<()> {
    let args = cli::Args::parse();

    let event_loop = WEL::EventLoop::new()?;

    let mut app = App {
        rctx: V::util::RenderContext::new(),
        rstate: RenderState::Suspended(None),
        scene: V::Scene::new(),
        camera: Camera::builder()
            .with_viewport_size(
                args.config.window.width as f64,
                args.config.window.height as f64,
            )
            .build(),

        elements: Vec::new(),
        config: args.config,
    };

    app.elements.push(
        Elem::builder()
            .with_shape(K::Rect::new(10., 10., 40., 40.))
            .with_style(EStyle::filled(V::peniko::color::palette::css::RED))
            .with_rotation(30.)
            .build(),
    );
    let _ = event_loop.run_app(&mut app);
    Ok(())
}
