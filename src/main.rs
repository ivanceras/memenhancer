extern crate unicode_width;
extern crate memenhancer;

use unicode_width::UnicodeWidthChar;
use unicode_width::UnicodeWidthStr;

fn main(){
    //let meme = r#"The rest of   凸(•̀_•́)凸❤️ ( ͡° ͜ʖ ͡°) \(°□°)/层∀   the text is here and there"#;
    let meme = include_str!("meme.me");
    println!("<meta charset='utf8'/>");
    println!("<pre>{}</prev>", meme);
    let x = 10;
    let y = 10;
    let text_width = 8.0;
    let text_height = 16.0;
    let svg = memenhancer::to_svg(meme, text_width, text_height);
    println!(r#"<svg x="100" y="200" xmlns="http://www.w3.org/2000/svg" font-family="arial" font-size="14" height="6976" width="1032">"#);
    println!("{}",get_styles());
    println!("{}", svg);
    println!("</svg>");
}

fn get_styles()->String{
let styles = r#"
<style xmlns="http://www.w3.org/2000/svg">

    line, path {
      stroke: black;
      stroke-width: 2;
      stroke-opacity: 1;
      fill-opacity: 1;
      stroke-linecap: round;
      stroke-linejoin: miter;
    }
    circle {
      stroke: black;
      stroke-width: 1;
      stroke-opacity: 1;
      fill-opacity: 1;
      stroke-linecap: round;
      stroke-linejoin: miter;
      fill:white;
    }
    
</style>
"#.into();
styles
}
