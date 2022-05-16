//general
pub mod basic {
    use const_format::concatcp;

    pub const BEL: &str = "\x07";
    pub const BS: &str = "\x08";
    pub const HT: &str = "\x09";
    pub const LF: &str = "\x0A";
    pub const VT: &str = "\x0B";
    pub const FF: &str = "\x0C";
    pub const CR: &str = "\x0D";
    pub const ESC: &str = "\x1B";
    pub const DEL: &str = "\x7F";
    pub const ESCB: &str = concatcp!(ESC, "[");
}

pub mod cursor {
    use const_format::concatcp;

    use crate::console::ansi::basic;

    pub const HOME: &str = concatcp! {basic::ESCB, "H"};

    pub fn move_to(line: i32, column: i32) -> String {
        format!("{}{};{}H", basic::ESCB, line, column)
    }

    pub fn move_up(lines: i32) -> String {
        format!("{}{}A", basic::ESCB, lines)
    }

    pub fn move_down(lines: i32) -> String {
        format!("{}{}B", basic::ESCB, lines)
    }

    pub fn move_right(columns: i32) -> String {
        format!("{}{}C", basic::ESCB, columns)
    }

    pub fn move_left(columns: i32) -> String {
        format!("{}{}D", basic::ESCB, columns)
    }

    pub fn move_start_up(lines: i32) -> String {
        format!("{}{}E", basic::ESCB, lines)
    }

    pub fn move_start_down(lines: i32) -> String {
        format!("{}{}F", basic::ESCB, lines)
    }

    pub fn move_column(column: i32) -> String {
        format!("{}{}G", basic::ESCB, column)
    }

    #[macro_export]
    macro_rules! move_start_down {
        ($lines:expr) => {{
            const LINES: i32 = $column;
            ::const_format::formatcp!("{}{}F", $crate::console::ansi::basic::ESCB, LINES)
        }};
    }

    #[macro_export]
    macro_rules! move_column {
        ($column:expr) => {{
            const COLUMN: i32 = $column;
            ::const_format::formatcp!("{}{}G", $crate::console::ansi::basic::ESCB, COLUMN)
        }};
    }

    pub const CURRENT_POSITION: &str = concatcp! {basic::ESCB, "6n"};
    pub const SCROLL_UP: &str = concatcp! {basic::ESCB, "M"};
    pub const SAVE_POSITION: &str = concatcp! {basic::ESCB, "7"};
    pub const RESTORE_POSITION: &str = concatcp! {basic::ESCB, "8"};
}

pub mod erase {
    use const_format::concatcp;

    use crate::console::ansi::basic;

    pub const ALL_AFTER_CUR: &str = concatcp! {basic::ESCB, "0J"};
    pub const ALL_BEFORE_CUR: &str = concatcp! {basic::ESCB, "1J"};
    pub const ALL: &str = concatcp! {basic::ESCB, "2J"};
    pub const SAVED_LINE: &str = concatcp! {basic::ESCB, "3J"};
    pub const LINE_AFTER_CUR: &str = concatcp! {basic::ESCB, "0K"};
    pub const LINE_BEFORE_CUR: &str = concatcp! {basic::ESCB, "1K"};
    pub const LINE_CUR: &str = concatcp! {basic::ESCB, "2K"};
}

pub mod font {
    use const_format::concatcp;

    use crate::console::ansi::basic;

    #[macro_export]
    macro_rules! set_graphics_mode {
        ($mode:expr) => {{
            const MODE: &str = $mode;
            ::const_format::formatcp!("{}1;34;{}m", $crate::console::ansi::basic::ESCB, MODE)
        }};
    }

    pub const RESET_ALL: &str = concatcp! {basic::ESCB,"0m"};
    pub const BOLD: &str = concatcp! {basic::ESCB,"1m"};
    pub const DIM: &str = concatcp! {basic::ESCB,"2m"};
    pub const ITALIC: &str = concatcp! {basic::ESCB, "3m"};
    pub const UNDERLINE: &str = concatcp! {basic::ESCB, "4m"};
    pub const BLINKING: &str = concatcp! {basic::ESCB, "5m"};
    pub const INVERT_COLOR: &str = concatcp! {basic::ESCB, "7m"};
    pub const INVISIBLE: &str = concatcp! {basic::ESCB, "8m"};
    pub const STRIKETHROUGH: &str = concatcp! {basic::ESCB, "9m"};
    pub const BOLD_DIM_RESET: &str = concatcp! {basic::ESCB, "22m"};
    pub const ITALIC_RESET: &str = concatcp! {basic::ESCB, "23m"};
    pub const UNDERLINE_RESET: &str = concatcp! {basic::ESCB, "24m"};
    pub const BLINKING_RESET: &str = concatcp! {basic::ESCB, "25m"};
    pub const INVERT_COLOR_RESET: &str = concatcp! {basic::ESCB, "27m"};
    pub const INVISIBLE_RESET: &str = concatcp! {basic::ESCB, "28m"};
    pub const STRIKETHROUGH_RESET: &str = concatcp! {basic::ESCB, "29m"};

    #[macro_export]
    macro_rules! set_foreground_color {
        ($red:expr, $green:expr, $blue:expr) => {{
            const RED: i32 = $red;
            const GREEN: i32 = $green;
            const BLUE: i32 = $blue;
            ::const_format::formatcp!(
                "{}38;2;{};{};{}m",
                $crate::console::ansi::basic::ESCB,
                RED,
                GREEN,
                BLUE
            )
        }};
    }

    #[macro_export]
    macro_rules! set_background_color {
        ($red:literal, $green:literal, $blue:literal) => {{
            const RED: i32 = $red;
            const GREEN: i32 = $green;
            const BLUE: i32 = $blue;
            ::const_format::formatcp!(
                "{}48;2;{};{};{}m",
                $crate::console::ansi::basic::ESCB,
                RED,
                GREEN,
                BLUE
            )
        }};
    }

    pub const RESET_FOREGROUND: &str = concatcp! {basic::ESCB, "1;39m"};
    pub const RESET_BACKGROUND: &str = concatcp! {basic::ESCB, "1;49m"};
}

pub mod misc {
    use const_format::concatcp;

    use crate::console::ansi::basic;

    pub const INVISIBLE_CUR: &str = concatcp! {basic::ESCB, "?25l"};
    pub const VISIBLE_CUR: &str = concatcp! {basic::ESCB, "?25h"};
    pub const LOAD_SCREEN: &str = concatcp! {basic::ESCB, "?47l"};
    pub const SAVE_SCREEN: &str = concatcp! {basic::ESCB, "?47h"};
    pub const ALT_BUFFER_ON: &str = concatcp! {basic::ESCB, "?1049h"};
    pub const ALT_BUFFER_OFF: &str = concatcp! {basic::ESCB, "?1049l"};
}
