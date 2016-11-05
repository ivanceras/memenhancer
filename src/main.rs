extern crate unicode_width;
extern crate memenhancer;

use unicode_width::UnicodeWidthChar;
use unicode_width::UnicodeWidthStr;

fn main(){
    let meme = r#" 凸(•̀_•́)凸❤️ ( ͡° ͜ʖ ͡°) \(°□°)/层∀ﾟ"#;
    println!("{}", meme);
    let x = 10;
    let y = 10;
    let text_width = 8.0;
    let text_height = 16.0;
    let svg = memenhancer::to_svg(meme, x, y, text_width, text_height);
    println!("<svg>{}</svg>", svg);
}
