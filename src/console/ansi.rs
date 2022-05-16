 
    //general
    pub mod basic {
        use const_format::concatcp;

        pub const BEL : &str = "\007";
        pub const BS : &str = "\010";
        pub const HT : &str = "\011";
        pub const LF : &str = "\012";
        pub const VT : &str = "\013";
        pub const FF : &str = "\014";
        pub const CR : &str = "\015";
        pub const ESC : &str = "\033";
        pub const DEL : &str = "\0177";
        pub const ESCB : &str = concatcp!(ESC, "[");
    }
    
    pub mod cursor {
        use const_format::concatcp;

        use crate::console::ansi::basic;

        
        pub const HOME : &str = concatcp!{basic::ESCB, "H"};
        
        pub fn move_to(line:i32, column:i32) -> String {
            format!("{}{};{}H",basic::ESCB, line, column)
        }
        
        pub fn move_up(lines:i32) -> String {
            format!("{}{}A",basic::ESCB, lines)
        }
        
        pub fn move_down(lines: i32) -> String {
            format!("{}{}B",basic::ESCB, lines)
        }
        
        pub fn move_right(columns:i32) -> String {
            format!("{}{}C",basic::ESCB, columns)
        }
        
        pub fn move_left(columns:i32) -> String {
            format!("{}{}D",basic::ESCB, columns)
        }
        
        pub fn move_start_up(lines:i32) -> String {
            format!("{}{}E",basic::ESCB, lines)
        }
        
        pub fn move_start_down(lines:i32) -> String {
            format!("{}{}F",basic::ESCB, lines)
        }
        
        pub fn move_column(column:i32) -> String {
            format!("{}{}G",basic::ESCB, column)
        }
        
        pub const CURRENT_POSITION : &str = concatcp!{basic::ESCB, "6n"};
        pub const SCROLL_UP : &str =concatcp!{basic::ESCB, "M"};
        pub const SAVE_POSITION : &str = concatcp!{basic::ESCB, "7"};
        pub const RESTORE_POSITION : &str = concatcp!{basic::ESCB, "8"};
    }
    
    pub mod erase {
        use const_format::concatcp;

        use crate::console::ansi::basic;

        
        pub const ALL_AFTER_CUR : &str = concatcp!{basic::ESCB, "0J"};
        pub const ALL_BEFORE_CUR : &str = concatcp!{basic::ESCB, "1J"};
        pub const ALL : &str = concatcp!{basic::ESCB, "2J"};
        pub const SAVED_LINE : &str = concatcp!{basic::ESCB, "3J"};
        pub const LINE_AFTER_CUR : &str = concatcp!{basic::ESCB, "0K"};
        pub const LINE_BEFORE_CUR : &str = concatcp!{basic::ESCB, "1K"};
        pub const LINE_CUR : &str = concatcp!{basic::ESCB, "2K"};
    }
    
    pub mod font {
        use const_format::concatcp;

        use crate::console::ansi::basic;

        
        
        pub fn set_graphics_mode(mode:&str) -> String {
            format!("{}1;34;{}m",basic::ESCB,mode)
        }
        
        pub const RESET_ALL : &str =concatcp!{basic::ESCB,"0m"};
        pub const BOLD : &str = concatcp!{basic::ESCB,"1m"};
        pub const DIM : &str = concatcp!{basic::ESCB,"2m"};
        pub const ITALIC : &str = concatcp!{basic::ESCB, "3m"};
        pub const UNDERLINE : &str = concatcp!{basic::ESCB, "4m"};
        pub const BLINKING : &str = concatcp!{basic::ESCB, "5m"};
        pub const INVERT_COLOR : &str = concatcp!{basic::ESCB, "7m"};
        pub const INVISIBLE : &str = concatcp!{basic::ESCB, "8m"};
        pub const STRIKETHROUGH : &str = concatcp!{basic::ESCB, "9m"};
        pub const BOLD_DIM_RESET : &str = concatcp!{basic::ESCB, "22m"};
        pub const ITALIC_RESET : &str = concatcp!{basic::ESCB, "23m"};
        pub const UNDERLINE_RESET : &str = concatcp!{basic::ESCB, "24m"};
        pub const BLINKING_RESET : &str = concatcp!{basic::ESCB, "25m"};
        pub const INVERT_COLOR_RESET : &str = concatcp!{basic::ESCB, "27m"};
        pub const INVISIBLE_RESET : &str = concatcp!{basic::ESCB, "28m"};
        pub const STRIKETHROUGH_RESET : &str = concatcp!{basic::ESCB, "29m"};
        
        #[macro_export]
        macro_rules! set_foreground_color {
            ($red:expr, $green:expr, $blue:expr,$a:tt) => {
                $crate::console::ansi::font::set_foreground_color_impl::<$a>($red, $green, $blue)
            };
        }
        pub fn set_foreground_color_impl<'a>(red:i32,green:i32,blue:i32) -> &'a str {
            format!("{}38;2;{};{};{}m",basic::ESCB ,red,green,blue).as_str()
            
        }
        
        pub fn set_background_color(red:i32,green:i32,blue:i32) -> String {
            format!("{}48;2;{};{};{}m",basic::ESCB ,red,green,blue)
        }
        
        pub const RESET_FOREGROUND : &str = concatcp!{basic::ESCB, "1;39m"};
        pub const RESET_BACKGROUND : &str = concatcp!{basic::ESCB, "1;49m"};
    }
    
    pub mod misc {
        use const_format::concatcp;

        use crate::console::ansi::basic;

        
        pub const INVISIBLE_CUR : &str = concatcp!{basic::ESCB, "?25l"};
        pub const VISIBLE_CUR : &str = concatcp!{basic::ESCB, "?25h"};
        pub const LOAD_SCREEN : &str = concatcp!{basic::ESCB, "?47l"};
        pub const SAVE_SCREEN : &str = concatcp!{basic::ESCB, "?47h"};
        pub const ALT_BUFFER_ON : &str = concatcp!{basic::ESCB, "?1049h"};
        pub const ALT_BUFFER_OFF : &str = concatcp!{basic::ESCB, "?1049l"};
    }