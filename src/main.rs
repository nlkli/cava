use vello::kurbo::Size;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::Key;
use winit::window::{Window, WindowAttributes, WindowId};

use vello::peniko::color::palette;
use vello::util::{RenderContext, RenderSurface};
use vello::wgpu::CurrentSurfaceTexture;
use vello::{AaConfig, Renderer, RendererOptions, Scene, kurbo, wgpu};

use vello::kurbo::{
    Affine, BezPath, Circle, CircleSegment, CubicBez, Ellipse, Line, Point, QuadBez, Rect,
    RoundedRect, Triangle, Vec2,
};

use std::sync::Arc;

mod camera;
mod cli;
mod el;

use camera::Camera;

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
    els: Vec<el::El>,
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
                    self.camera.state_mut().viewport =
                        Size::new(size.width as f64, size.height as f64);
                    window.request_redraw();
                }
            }
            WindowEvent::KeyboardInput { event, .. } if event.state == ElementState::Pressed => {
                match event.logical_key.as_ref() {
                    Key::Character(char) => {
                        match char {
                            "j" => self.camera.state_mut().zoom += 0.2,
                            "k" => self.camera.state_mut().zoom -= 0.2,
                            "h" => self.camera.state_mut().position.x += 2.1,
                            "l" => self.camera.state_mut().position.x -= 2.1,
                            _ => {}
                        }
                        window.request_redraw();
                    }
                    _ => {}
                }
            }
            WindowEvent::RedrawRequested => {
                self.scene.reset();

                self.camera.render(&mut self.scene, &self.els);

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
        els: Vec::new(),
    };

    app.els = vec![
        el::rect(
            Rect::new(0., 0., 100., 100.),
            el::Style::filled(palette::css::RED),
            None,
        ),
        el::circle(
            Circle::new(Point::ORIGIN, 40.),
            el::Style::stroked(palette::css::BLUE, 3.0),
            Some(Affine::translate((200., 50.))),
        ),
        el::ellipse(
            Ellipse::new(Point::ORIGIN, Vec2::new(60., 30.), 0.3),
            el::Style::filled_and_stroked(palette::css::GREEN, palette::css::DARK_GREEN, 2.0),
            Some(Affine::translate((200., 150.))),
        ),
        el::rounded_rect(
            RoundedRect::new(0., 0., 120., 80., 12.),
            el::Style::stroked(palette::css::GOLD, 4.0),
            Some(Affine::translate((0., 250.))),
        ),
        el::triangle(
            Triangle::new(
                Point::new(0., -50.),
                Point::new(-50., 40.),
                Point::new(50., 40.),
            ),
            el::Style::filled(palette::css::PURPLE),
            Some(Affine::translate((400., 100.)) * Affine::rotate(0.5)),
        ),
        el::line(
            Line::new(Point::new(0., 0.), Point::new(150., 100.)),
            el::Style::stroked(palette::css::ORANGE, 5.0),
            Some(Affine::translate((400., 250.))),
        ),
        el::cubic_bez(
            CubicBez::new(
                Point::new(0., 0.),
                Point::new(30., -80.),
                Point::new(90., 80.),
                Point::new(120., 0.),
            ),
            el::Style::stroked(palette::css::DEEP_PINK, 2.5),
            Some(Affine::translate((600., 250.))),
        ),
        el::quad_bez(
            QuadBez::new(
                Point::new(0., 0.),
                Point::new(50., -60.),
                Point::new(100., 0.),
            ),
            el::Style::stroked(palette::css::GRAY, 2.0),
            Some(Affine::translate((0., 400.))),
        ),
        el::circle_segment(
            CircleSegment::new(Point::ORIGIN, 70., 30., 0.0, std::f64::consts::FRAC_PI_3),
            el::Style::filled(palette::css::LIGHT_SKY_BLUE),
            Some(Affine::translate((250., 400.))),
        ),
        el::bez_path(
            {
                let mut path = BezPath::new();
                let n = 5;
                let outer = 60.0;
                let inner = 25.0;
                for i in 0..(n * 2) {
                    let r = if i % 2 == 0 { outer } else { inner };
                    let theta =
                        std::f64::consts::PI * i as f64 / n as f64 - std::f64::consts::FRAC_PI_2;
                    let pt = Point::new(r * theta.cos(), r * theta.sin());
                    if i == 0 {
                        path.move_to(pt);
                    } else {
                        path.line_to(pt);
                    }
                }
                path.close_path();
                path
            },
            el::Style::filled_and_stroked(palette::css::YELLOW, palette::css::SADDLE_BROWN, 2.0),
            Some(Affine::translate((450., 400.))),
        ),
        el::rect(
            Rect::new(0., 0., 40., 40.),
            el::Style::filled(palette::css::CRIMSON.with_alpha(0.4)),
            Some(Affine::translate((650., 400.)) * Affine::scale(3.0)),
        ),
    ];
    let _ = event_loop.run_app(&mut app);
}
