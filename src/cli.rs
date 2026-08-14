use crate::config::Config;

const VERSION: &str = "cava 0.1.0 [https://github.com/nlkli/cava]";
const HELP: &str = r#"cava - visualizer
Usage: cava [OPTIONS] [CONFIG_PATH]
Options:
  -t, --title <TITLE>                       Window title
  -w, --width <WIDTH>                       Window width
  -h, --height <HEIGHT>                     Window height
  -x <X>                                    Window X position
  -y <Y>                                    Window Y position
      --position <X,Y>                      Window position
  -f, --fullscreen                          Toggle fullscreen
  -r, --resizable                           Toggle resizable
  -d, --decorations                         Toggle decorations
  -T, --transparent                         Toggle transparent window
  -m, --maximized                           Toggle maximized
      --min-width <WIDTH>                   Minimum window width
      --min-height <HEIGHT>                 Minimum window height
      --max-width <WIDTH>                   Maximum window width
      --max-height <HEIGHT>                 Maximum window height
      --macos-titlebar-transparent          Toggle macOS transparent titlebar
      --macos-titlebar-hidden               Toggle macOS hidden titlebar
      --macos-titlebar-buttons-hidden       Toggle macOS hidden titlebar buttons
      --macos-title-hidden                  Toggle macOS hidden title
      --macos-fullsize-content-view         Toggle macOS fullsize content view
      --macos-has-shadow                    Toggle macOS window shadow
      --macos-option-as-alt <MODE>          macOS Option-as-Alt mode (none|only_left|only_right|both)
      --macos-tabbing-identifier <ID>       macOS tabbing identifier
      --macos-borderless-game               Toggle macOS borderless game mode
      --macos-disallow-hidpi                Toggle macOS disallow HiDPI
  -V, --version                             Print version
  -H, --help                                Print help"#;

#[derive(Clone, Debug)]
pub struct Args {
    pub config: Config,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            config: Config::default(),
        }
    }
}

impl Args {
    pub fn parse() -> Self {
        let mut args = Self::default();
        let c = &mut args.config;

        let mut last: Option<&'static str> = None;

        let mut iter = std::env::args().skip(1);

        while let Some(arg) = iter.next() {
            if let Some(flag) = arg.strip_prefix("--") {
                match flag {
                    "title" => last = Some("title"),
                    "width" => last = Some("width"),
                    "height" => last = Some("height"),
                    "x" => last = Some("x"),
                    "y" => last = Some("y"),
                    "position" => last = Some("position"),
                    "min-width" => last = Some("min-width"),
                    "min-height" => last = Some("min-height"),
                    "max-width" => last = Some("max-width"),
                    "max-height" => last = Some("max-height"),
                    "macos-option-as-alt" => last = Some("macos-option-as-alt"),
                    "macos-tabbing-identifier" => last = Some("macos-tabbing-identifier"),

                    "fullscreen" => c.window.fullscreen = !c.window.fullscreen,
                    "resizable" => c.window.resizable = !c.window.resizable,
                    "decorations" => c.window.decorations = !c.window.decorations,
                    "transparent" => c.window.transparent = !c.window.transparent,
                    "maximized" => c.window.maximized = !c.window.maximized,

                    "macos-titlebar-transparent" => {
                        c.window.macos_titlebar_transparent = !c.window.macos_titlebar_transparent
                    }
                    "macos-titlebar-hidden" => {
                        c.window.macos_titlebar_hidden = !c.window.macos_titlebar_hidden
                    }
                    "macos-titlebar-buttons-hidden" => {
                        c.window.macos_titlebar_buttons_hidden =
                            !c.window.macos_titlebar_buttons_hidden
                    }
                    "macos-title-hidden" => {
                        c.window.macos_title_hidden = !c.window.macos_title_hidden
                    }
                    "macos-fullsize-content-view" => {
                        c.window.macos_fullsize_content_view = !c.window.macos_fullsize_content_view
                    }
                    "macos-has-shadow" => c.window.macos_has_shadow = !c.window.macos_has_shadow,
                    "macos-borderless-game" => {
                        c.window.macos_borderless_game = !c.window.macos_borderless_game
                    }
                    "macos-disallow-hidpi" => {
                        c.window.macos_disallow_hidpi = !c.window.macos_disallow_hidpi
                    }

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
                for c_flag in flags.chars() {
                    match c_flag {
                        't' => last = Some("title"),
                        'w' => last = Some("width"),
                        'h' => last = Some("height"),
                        'x' => last = Some("x"),
                        'y' => last = Some("y"),
                        'f' => c.window.fullscreen = !c.window.fullscreen,
                        'r' => c.window.resizable = !c.window.resizable,
                        'd' => c.window.decorations = !c.window.decorations,
                        'T' => c.window.transparent = !c.window.transparent,
                        'm' => c.window.maximized = !c.window.maximized,

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
                    Some("title") => {
                        c.window.title = arg;
                    }

                    Some("width") => {
                        if let Ok(width) = arg.parse::<u32>() {
                            c.window.width = width;
                        }
                    }

                    Some("height") => {
                        if let Ok(height) = arg.parse::<u32>() {
                            c.window.height = height;
                        }
                    }

                    Some("x") => {
                        if let Ok(x) = arg.parse::<u32>() {
                            c.window.x = x;
                        }
                    }

                    Some("y") => {
                        if let Ok(y) = arg.parse::<u32>() {
                            c.window.y = y;
                        }
                    }

                    Some("position") => {
                        let parts: Vec<&str> = arg.split(',').collect();
                        if parts.len() == 2 {
                            if let (Ok(x), Ok(y)) =
                                (parts[0].trim().parse(), parts[1].trim().parse())
                            {
                                c.window.position = Some((x, y));
                            }
                        }
                    }

                    Some("min-width") => {
                        if let Ok(v) = arg.parse::<u32>() {
                            c.window.min_width = v;
                        }
                    }

                    Some("min-height") => {
                        if let Ok(v) = arg.parse::<u32>() {
                            c.window.min_height = v;
                        }
                    }

                    Some("max-width") => {
                        if let Ok(v) = arg.parse::<u32>() {
                            c.window.max_width = v;
                        }
                    }

                    Some("max-height") => {
                        if let Ok(v) = arg.parse::<u32>() {
                            c.window.max_height = v;
                        }
                    }

                    Some("macos-option-as-alt") => {
                        c.window.macos_option_as_alt = arg;
                    }

                    Some("macos-tabbing-identifier") => {
                        c.window.macos_tabbing_identifier = arg;
                    }

                    None => {
                        *c = Config::from_ini_file(&arg, Some(c.clone())).unwrap();
                    }

                    _ => (),
                }
            }
        }

        args
    }
}
