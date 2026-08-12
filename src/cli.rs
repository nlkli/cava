use winit::{
    dpi::{LogicalPosition, LogicalSize},
    window::{Fullscreen, Window, WindowAttributes},
};

const VERSION: &str = "cava 0.1.0 [https://github.com/nlkli/cava]";
const HELP: &str = r#"cava - visualizer
Usage: cava [OPTIONS]
Options:
  -t, --title <TITLE>     Window title
  -w, --width <WIDTH>     Window width
  -h, --height <HEIGHT>   Window height
  -x <X>, -y <Y>          Window XY position
  -f, --fullscreen        Start fullscreen
  -r, --resizable         Allow resizing
  -d, --decorations       Show window decorations
  -T, --transparent       Transparent window
  -V, --version           
  -H, --help"#;

#[derive(Clone, Debug)]
pub struct Args {
    /// Window title
    pub title: String,

    /// Window width
    pub width: u32,

    /// Window height
    pub height: u32,

    /// Window X position
    pub x: Option<i32>,

    /// Window Y position
    pub y: Option<i32>,

    /// Start in fullscreen
    pub fullscreen: bool,

    /// Allow window resizing
    pub resizable: bool,

    /// Show window decorations
    pub decorations: bool,

    /// Use a transparent window
    pub transparent: bool,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            title: "cava".into(),
            width: 800,
            height: 600,
            x: None,
            y: None,
            fullscreen: false,
            resizable: true,
            decorations: true,
            transparent: false,
        }
    }
}

impl Args {
    pub fn parse() -> Self {
        let mut args = Self::default();
        let mut last: Option<char> = None;

        let mut iter = std::env::args().skip(1);

        while let Some(arg) = iter.next() {
            if let Some(flag) = arg.strip_prefix("--") {
                match flag {
                    "title" => last = Some('t'),
                    "width" => last = Some('w'),
                    "height" => last = Some('h'),
                    "x" => last = Some('x'),
                    "y" => last = Some('y'),

                    "fullscreen" => args.fullscreen = true,
                    "resizable" => args.resizable = true,
                    "decorations" => args.decorations = true,
                    "transparent" => args.transparent = true,

                    "help" => {
                        println!("{HELP}");
                        std::process::exit(0);
                    }

                    "version" => {
                        println!("{VERSION}");
                        std::process::exit(0);
                    }

                    _ => (),
                }
            } else if let Some(flags) = arg.strip_prefix('-') {
                for c in flags.chars() {
                    match c {
                        't' | 'w' | 'h' | 'x' | 'y' => last = Some(c),
                        'f' => args.fullscreen = true,
                        'r' => args.resizable = true,
                        'd' => args.decorations = true,
                        'T' => args.transparent = true,

                        'H' => {
                            println!("{HELP}");
                            std::process::exit(0);
                        }

                        'V' => {
                            println!("{VERSION}");
                            std::process::exit(0);
                        }

                        _ => (),
                    }
                }
            } else {
                match last.take() {
                    Some('t') => {
                        args.title = arg;
                    }

                    Some('w') => {
                        if let Ok(width) = arg.parse::<u32>() {
                            args.width = width;
                        }
                    }

                    Some('h') => {
                        if let Ok(height) = arg.parse::<u32>() {
                            args.height = height;
                        }
                    }

                    Some('x') => {
                        if let Ok(x) = arg.parse::<i32>() {
                            args.x = Some(x);
                        }
                    }

                    Some('y') => {
                        if let Ok(y) = arg.parse::<i32>() {
                            args.y = Some(y);
                        }
                    }

                    _ => (),
                }
            }
        }

        args
    }

    pub fn window_attributes(&self) -> WindowAttributes {
        let mut attributes = Window::default_attributes()
            .with_title(&self.title)
            .with_inner_size(LogicalSize::new(self.width, self.height))
            .with_resizable(self.resizable)
            .with_decorations(self.decorations)
            .with_transparent(self.transparent)
            .with_fullscreen(if self.fullscreen {
                Some(Fullscreen::Borderless(None))
            } else {
                None
            });

        if let (Some(x), Some(y)) = (self.x, self.y) {
            attributes = attributes.with_position(LogicalPosition::new(x, y));
        }

        attributes
    }
}
