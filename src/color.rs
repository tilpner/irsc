pub const WHITE: &'static str = "00";
pub const BLACK: &'static str = "01";
pub const BLUE: &'static str = "02";
pub const GREEN: &'static str = "03";
pub const RED: &'static str = "04";
pub const BROWN: &'static str = "05";
pub const PURPLE: &'static str = "06";
pub const ORANGE: &'static str = "07";
pub const YELLOW: &'static str = "08";
pub const LIME: &'static str = "09";
pub const TEAL: &'static str = "10";
pub const LIGHT_CYAN: &'static str = "11";
pub const LIGHT_BLUE: &'static str = "12";
pub const PINK: &'static str = "13";
pub const GREY: &'static str = "14";
pub const LIGHT_GREY: &'static str = "15";

pub const TRANSPARENT: &'static str = "99";

pub fn normal(s: &str) -> String {
    format!("\x0F{}\x0F", s)
}

pub fn bold(s: &str) -> String {
    format!("\x02{}\x02", s)
}

pub fn italic(s: &str) -> String {
    format!("\x1D{}\x1D", s)
}

pub fn underline(s: &str) -> String {
    format!("\x1F{}\x1F", s)
}

pub fn foreground(s: &str, foreground: &str) -> String {
    format!("\x03{}{}\x03", foreground, s)
}

pub fn background(s: &str, background: &str) -> String {
    format!("\x03,{}{}\x03", background, s)
}

pub fn color(s: &str, foreground: &str, background: &str) -> String {
    format!("\x03{},{}{}\x03", foreground, background, s)
}
