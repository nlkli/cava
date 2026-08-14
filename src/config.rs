use anyhow::{Context, Result};
use configparser::ini::Ini;

macro_rules! parse_string {
    ($ini:expr, $section:expr, $key:expr, $target:expr) => {
        if let Some(v) = $ini.get($section, $key) {
            $target = v;
        }
    };
}

macro_rules! parse_number {
    ($ini:expr, $section:expr, $key:expr, $target:expr, $type:ty) => {
        if let Some(v) = $ini.get($section, $key) {
            $target = v.parse::<$type>().context(format!(
                "Failed to parse '{}' as {}",
                v,
                stringify!($type)
            ))?;
        }
    };
}

macro_rules! parse_bool {
    ($ini:expr, $section:expr, $key:expr, $target:expr) => {
        if let Some(v) = $ini
            .getboolcoerce($section, $key)
            .map_err(anyhow::Error::msg)?
        {
            $target = v;
        }
    };
}

#[derive(Clone, Debug)]
pub struct Window {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub x: u32,
    pub y: u32,
    pub position: Option<(u32, u32)>,
    pub fullscreen: bool,
    pub resizable: bool,
    pub decorations: bool,
    pub transparent: bool,
    pub maximized: bool,
    pub min_width: u32,
    pub min_height: u32,
    pub max_width: u32,
    pub max_height: u32,

    // macOS
    pub macos_titlebar_transparent: bool,
    pub macos_titlebar_hidden: bool,
    pub macos_titlebar_buttons_hidden: bool,
    pub macos_title_hidden: bool,
    pub macos_fullsize_content_view: bool,
    pub macos_has_shadow: bool,
    pub macos_option_as_alt: String,
    pub macos_tabbing_identifier: String,
    pub macos_borderless_game: bool,
    pub macos_disallow_hidpi: bool,
    // // Windows
    // pub windows_skip_taskbar: bool,
    // pub windows_undecorated_shadow: bool,
    // pub windows_corner_preference: Option<String>,
    // pub windows_class_name: Option<String>,
    // pub windows_border_color: Option<String>,
    // pub windows_title_background_color: Option<String>,
    // pub windows_title_text_color: Option<String>,

    // // X11
    // pub x11_general_name: Option<String>,
    // pub x11_instance_name: Option<String>,
    // pub x11_override_redirect: bool,
    // pub x11_window_type: Option<String>,
    // pub x11_visual_id: Option<u32>,
    // pub x11_screen_id: Option<i32>,
    // pub x11_embed_parent_window: Option<u32>,

    // // Wayland
    // pub wayland_general_name: Option<String>,
    // pub wayland_instance_name: Option<String>,
    // pub wayland_activation_token: Option<String>,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            title: "cava".into(),
            width: 800,
            height: 600,
            x: 0,
            y: 0,
            position: None,
            fullscreen: false,
            resizable: true,
            decorations: true,
            transparent: false,
            maximized: false,
            min_width: 0,
            min_height: 0,
            max_width: 0,
            max_height: 0,

            // macOS
            macos_titlebar_transparent: false,
            macos_titlebar_hidden: false,
            macos_titlebar_buttons_hidden: false,
            macos_title_hidden: false,
            macos_fullsize_content_view: false,
            macos_has_shadow: true,
            macos_option_as_alt: "".into(),
            macos_tabbing_identifier: "".into(),
            macos_borderless_game: false,
            macos_disallow_hidpi: false,
            // // Windows
            // windows_skip_taskbar: false,
            // windows_undecorated_shadow: true,
            // windows_corner_preference: None,
            // windows_class_name: None,
            // windows_border_color: None,
            // windows_title_background_color: None,
            // windows_title_text_color: None,

            // // X11
            // x11_general_name: None,
            // x11_instance_name: None,
            // x11_override_redirect: false,
            // x11_window_type: None,
            // x11_visual_id: None,
            // x11_screen_id: None,
            // x11_embed_parent_window: None,

            // // Wayland
            // wayland_general_name: None,
            // wayland_instance_name: None,
            // wayland_activation_token: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub window: Window,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window: Window::default(),
        }
    }
}

impl Config {
    pub fn from_ini_file(
        path: impl AsRef<std::path::Path>,
        default: Option<Config>,
    ) -> Result<Self> {
        let mut ini = Ini::new();
        ini.load(path).map_err(|e| anyhow::anyhow!("{e}"))?;

        let mut config = default.unwrap_or_default();

        parse_string!(ini, "window", "title", config.window.title);
        parse_number!(ini, "window", "width", config.window.width, u32);
        parse_number!(ini, "window", "height", config.window.height, u32);
        parse_bool!(ini, "window", "fullscreen", config.window.fullscreen);
        parse_bool!(ini, "window", "resizable", config.window.resizable);
        parse_bool!(ini, "window", "decorations", config.window.decorations);
        parse_bool!(ini, "window", "transparent", config.window.transparent);
        parse_bool!(ini, "window", "maximized", config.window.maximized);
        parse_number!(ini, "window", "min_width", config.window.min_width, u32);
        parse_number!(ini, "window", "min_height", config.window.min_height, u32);
        parse_number!(ini, "window", "max_width", config.window.max_width, u32);
        parse_number!(ini, "window", "max_height", config.window.max_height, u32);

        parse_bool!(
            ini,
            "window",
            "macos_titlebar_transparent",
            config.window.macos_titlebar_transparent
        );
        parse_bool!(
            ini,
            "window",
            "macos_titlebar_hidden",
            config.window.macos_titlebar_hidden
        );
        parse_bool!(
            ini,
            "window",
            "macos_titlebar_buttons_hidden",
            config.window.macos_titlebar_buttons_hidden
        );
        parse_bool!(
            ini,
            "window",
            "macos_title_hidden",
            config.window.macos_title_hidden
        );
        parse_bool!(
            ini,
            "window",
            "macos_fullsize_content_view",
            config.window.macos_fullsize_content_view
        );
        parse_bool!(
            ini,
            "window",
            "macos_has_shadow",
            config.window.macos_has_shadow
        );
        parse_bool!(
            ini,
            "window",
            "macos_borderless_game",
            config.window.macos_borderless_game
        );
        parse_bool!(
            ini,
            "window",
            "macos_disallow_hidpi",
            config.window.macos_disallow_hidpi
        );
        parse_string!(
            ini,
            "window",
            "macos_option_as_alt",
            config.window.macos_option_as_alt
        );
        parse_string!(
            ini,
            "window",
            "macos_tabbing_identifier",
            config.window.macos_tabbing_identifier
        );

        if let Some(pos) = ini.get("window", "position") {
            if pos.to_lowercase() != "none" && !pos.is_empty() {
                let parts: Vec<&str> = pos.split(',').collect();
                anyhow::ensure!(
                    parts.len() == 2,
                    "Invalid position '{}'. Expected 'x,y'",
                    pos
                );
                config.window.position = Some((
                    parts[0]
                        .trim()
                        .parse()
                        .context(format!("Invalid x in '{}'", pos))?,
                    parts[1]
                        .trim()
                        .parse()
                        .context(format!("Invalid y in '{}'", pos))?,
                ));
            }
        }

        Ok(config)
    }

    pub fn window_attributes(&self) -> winit::window::WindowAttributes {
        let w = &self.window;

        let mut attr = winit::window::Window::default_attributes()
            .with_title(&w.title)
            .with_inner_size(winit::dpi::LogicalSize::new(w.width, w.height))
            .with_resizable(w.resizable)
            .with_decorations(w.decorations)
            .with_transparent(w.transparent)
            .with_maximized(w.maximized)
            .with_fullscreen(if w.fullscreen {
                Some(winit::window::Fullscreen::Borderless(None))
            } else {
                None
            });

        if w.x > 0 || w.y > 0 {
            attr = attr.with_position(winit::dpi::LogicalPosition::new(w.x, w.y));
        }

        if let Some(p) = w.position {
            attr = attr.with_position(winit::dpi::LogicalPosition::new(p.0, p.1));
        }

        let mut min_inner_size = winit::dpi::LogicalSize::new(1, 1);
        if w.min_width > 0 {
            let min_width = w.min_width.max(1);
            min_inner_size.width = min_width;
        }
        if w.min_height > 0 {
            let min_height = w.min_height.max(1);
            min_inner_size.height = min_height;
        }
        attr = attr.with_min_inner_size(min_inner_size);

        if w.max_width > 0 || w.max_height > 0 {
            let mut max_inner_size = winit::dpi::LogicalSize::new(u32::MAX, u32::MAX);
            if w.max_width > 0 {
                max_inner_size.width = w.max_width.max(2);
            }
            if w.max_height > 0 {
                max_inner_size.height = w.max_height.max(2);
            }
            attr = attr.with_max_inner_size(max_inner_size);
        }

        #[cfg(target_os = "macos")]
        {
            use winit::platform::macos::WindowAttributesExtMacOS;

            attr = attr
                .with_titlebar_transparent(w.macos_titlebar_transparent)
                .with_titlebar_hidden(w.macos_titlebar_hidden)
                .with_titlebar_buttons_hidden(w.macos_titlebar_buttons_hidden)
                .with_title_hidden(w.macos_title_hidden)
                .with_fullsize_content_view(w.macos_fullsize_content_view)
                .with_has_shadow(w.macos_has_shadow)
                .with_borderless_game(w.macos_borderless_game)
                .with_disallow_hidpi(w.macos_disallow_hidpi);

            if !w.macos_option_as_alt.is_empty() {
                let option = match w.macos_option_as_alt.to_lowercase().as_str() {
                    "none" => winit::platform::macos::OptionAsAlt::None,
                    "only_left" => winit::platform::macos::OptionAsAlt::OnlyLeft,
                    "only_right" => winit::platform::macos::OptionAsAlt::OnlyRight,
                    "both" => winit::platform::macos::OptionAsAlt::Both,
                    _ => winit::platform::macos::OptionAsAlt::None,
                };
                attr = attr.with_option_as_alt(option);
            }

            if !w.macos_tabbing_identifier.is_empty() {
                attr = attr.with_tabbing_identifier(&w.macos_tabbing_identifier);
            }
        }

        attr
    }
}
