extern crate unicode_width;
extern crate svg;


use unicode_width::UnicodeWidthStr;
use unicode_width::UnicodeWidthChar;
use svg::node::element::Circle as SvgCircle;
use svg::node::element::Text as SvgText;
use svg::Node;


pub struct Settings {
    text_width: f32,
    text_height: f32,
}

impl Settings{
    
    fn offset(&self)->(f32, f32){
        (0.0, self.text_height * 2.0)
    }
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            text_width: 8.0,
            text_height: 16.0,
        }
    }
}

enum Anchor{
    Start,
    Middle,
    End
}


/// The whole meme body
/// 
#[derive(Clone,Debug)]
struct Meme{
    /// location to the left until a space is encountered
    start_position: usize,
    head: Head,
    /// location to the right until a space is encoutered
    end_position: usize,
    /// string at the left
    left_side: String,
    /// string at the right
    right_side: String
}

impl Meme{
    
    fn get_svg_elements(&self, settings: &Settings) -> Vec<Box<Node>>{
        let mut elements = vec![];
        elements.extend(self.head.get_svg_elements(settings));
        let right_text = to_svg_text(&self.right_side, self.head.endx, 0, settings, Anchor::Start);
        let left_text = to_svg_text(&self.left_side, self.head.startx - 1, 0, settings, Anchor::End);
        elements.push(Box::new(left_text));
        elements.push(Box::new(right_text));
        elements
    }

    
}


fn to_svg_text(s: &str, x: usize, y: usize, settings: &Settings, anchor: Anchor) -> SvgText {
    let (offsetx, offsety) = settings.offset();
    let sx = x as f32 * settings.text_width + settings.text_width / 4.0 + offsetx;
    let sy = y as f32 * settings.text_height + settings.text_height * 3.0 / 4.0 + offsety;
    let mut svg_text = SvgText::new()
        .set("x", sx)
        .set("y", sy);
    match anchor{
        Anchor::Start => {
            svg_text.assign("text-anchor", "start");
        }
        Anchor::Middle => {
            svg_text.assign("text-anchor", "middle");
        }
        Anchor::End => {
            svg_text.assign("text-anchor", "end");
        }
    };

    let text_node = svg::node::Text::new(escape_str(s));
    svg_text.append(text_node);
    svg_text
}


/// The head of the meme
/// the face is the string in between
/// used in detecting if it's a valid meme or not
#[derive(Clone,Debug)]
struct Head{
    // character position
    start_position: usize,
    // left x location x1
    startx: usize,
    face: String,
    // right x location x2
    endx: usize,
    // end position
    end_position: usize
}

impl Head{

    fn is_meme_face(&self) -> bool {
        is_meme(&self.face)
    }

    fn distance(&self) -> usize {
        self.endx - self.startx     
    }

    fn face_width(&self) -> usize{
        self.face.width()
    }
    
    fn get_svg_elements(&self, settings:&Settings) -> Vec<Box<Node>> {
        let mut elements: Vec<Box<Node>> = vec![];
        elements.push(Box::new(self.get_circle(settings)));
        elements.push(Box::new(self.get_face_text(settings)));
        elements
    }

    fn get_face_text(&self, settings: &Settings) -> SvgText{
        to_svg_text(&self.face, self.startx + 1, 0, settings, Anchor::Start)
    }

    fn get_circle(&self, settings: &Settings)-> SvgCircle{
        let (offsetx, offsety) = settings.offset();
        let text_width = settings.text_width;
        let text_height = settings.text_height;
        let xloc = self.startx as f32 * text_width;
        let radius = self.distance() as f32 / 2.0;
        let startx = self.startx as f32;
        let endx = self.endx as f32;
        let center = startx + radius;
        let cx = center * text_width; 
        let cy = text_height / 2.0 + offsety;
        let cr = radius * text_width;

        SvgCircle::new()
            .set("cx",cx)
            .set("cy", cy)
            .set("r", cr)
    }

}

#[derive(Debug)]
pub struct Circle{
    x: f32,
    y: f32,
    r: f32,
}


/// detect whether the series of string could be a meme
/// has at least 1 full width character (width = 2)
/// has at least 1 zero sized width character (width = 0)
/// has at least 1 character that has more than 1 byte in size
/// unicode value is way up high
pub fn is_meme(ch: &str) -> bool{
    let total_bytes = ch.len();
    let total_width = ch.width();
    let mut gte_bytes2 = 0; 
    let mut gte_width2 = 0;
    let mut zero_width = 0;
    let mut gte_unicode_1k = 0;
    for c in ch.chars(){
        if c as u32 >= 1000{
            gte_unicode_1k += 1;
        }
        if c.len_utf8() >= 2{
            gte_bytes2 += 1;
        }
        if let Some(uw) = c.width(){
            if uw >= 2 {
                gte_width2 += 1;
            }
            if uw == 0 {
                zero_width += 1;
            }
        }
    }
    /*
    println!("total_bytes: {}", total_bytes);
    println!("gte_bytes2: {}", gte_bytes2);
    println!("gte_width2: {}", gte_width2);
    println!("zero_width: {}", zero_width);
    println!("gte_unicode_1k {}", gte_unicode_1k);
    println!("total_width: {}", total_width);
    println!("");
    */
    gte_bytes2 > 0 || gte_width2 > 0
    || zero_width > 0 || gte_unicode_1k > 0
    || total_bytes > total_width
}

pub fn to_svg(s: &str, text_width: f32, text_height: f32) -> String {
    let elements = get_svg_elements(s, text_width, text_height);
    let mut svg = String::new();
    for elm in elements{
        svg.push_str(&elm.to_string())
    }
    svg
}

pub fn get_svg_elements(s: &str, text_width: f32, text_height: f32) -> Vec<Box<Node>> {
    let settings = Settings {
                    text_width: text_width,
                    text_height: text_height
                   };
    let memes = parse_memes(s);
    let mut svg = vec![];
    for meme in memes{
        let mel = meme.get_svg_elements(&settings);
        svg.extend(mel);
    }
    svg
}

fn circle_to_svg(circle: &Circle) -> SvgCircle {
    SvgCircle::new() 
        .set("cx", circle.x)
        .set("cy", circle.y)
        .set("r", circle.r)
}

pub fn get_circles(s: &str, x: usize, y:usize, 
        text_width: f32, text_height: f32) -> Vec<Circle>{
    let mut circles = vec![];
    let xloc = x as f32 * text_width;
    let yloc = y as f32 * text_height;
    let memes = parse_memes(s);
    for m in memes{
        let head = m.head;
        let radius = head.face_width() as f32 / 2.0 + 0.5;
        let startx = head.startx as f32;
        let endx = head.endx as f32;
        let left_adjustment = 0.5 + startx * 0.1;
        let right_adjustment = 0.5 + endx * 0.1;
        let center = 0.5 + startx + radius;
        let comp_center = center * text_width + xloc;
        let circle = Circle{
            x: comp_center,
            y: yloc + text_height / 2.0,
            r: radius * text_width
        };
        circles.push(circle);
    }
    circles
}



fn parse_memes(s: &str) -> Vec<Meme> {
    let mut memes = vec![];
    let mut paren_opened = false;
    let mut meme_face = String::new();
    let mut index = 0;
    let mut total_width = 0;
    let mut face_markers:Vec<Head> = vec![];
    let mut startx = 0;
    let mut start_position = 0;
    let mut meme_start = 0;
    let mut meme_end = 0;
    let mut meme_body = String::new();
    let mut meme_left_side = String::new();
    let mut meme_right_side = String::new();
    let mut meme_head = None;
    let total_chars = s.chars().count();
    for ch in s.chars(){
        let last_char = index == total_chars - 1;
        if meme_head.is_none() && ch == ' '{
            meme_start = index + 1;
            meme_body.clear();
        }
        if meme_head.is_some() && (ch == ' ' || last_char){
            meme_end = index;     
            let meme = Meme{
               start_position: meme_start,  
               head: meme_head.clone().unwrap(),
               end_position: meme_end,
               left_side: meme_left_side.clone(), 
               right_side: meme_right_side.clone(),
            };
            memes.push(meme);
            meme_right_side.clear();
            meme_left_side.clear();
            meme_body.clear();
            meme_head = None;
        }
        if meme_head.is_some(){
            meme_right_side.push(ch);
        }

        if paren_opened && ch == ')'{ //if paren_opened and encountered a closing
            paren_opened  = false;
            let head = Head{
                start_position: start_position,
                startx: startx,
                face: meme_face.clone(),
                end_position: index,
                endx: total_width,
            };
            meme_head = Some(head.clone());
            face_markers.push(head);
            meme_face.clear();
        }
        if paren_opened{
           meme_face.push(ch); 
        }
        if ch == '('{
            paren_opened = true;
            startx = total_width;
            start_position = index;
            meme_left_side = meme_body.clone();
            meme_face.clear();
        }
        meme_body.push(ch);
        if let Some(uw) = ch.width(){
            total_width += uw;
        }
        index += 1;
    } 
    memes
}

#[test]
fn test_meme() {
    assert!(!is_meme(" -_- "));
    assert!(!is_meme("     "));
    assert!(is_meme(" ͡° ͜ʖ ͡°"));
    assert!(is_meme("⌐■_■"));
    assert!(is_meme("ツ"));
}

#[test]
fn test_bound(){
    let meme = "(♥_♥)";
    let memes = parse_memes(meme); 
    for m in memes{
        let b = m.head;
        println!("bound {:?} d:{} w:{} ", b, b.distance(), b.face_width());
        assert_eq!(3, b.face_width());
    }
}

fn escape_str(s: &str) -> String{
    let mut escaped = String::new();
    for c in s.chars(){
        escaped.push_str(&escape_char(&c));    
    }
    escaped
}

fn escape_char(ch: &char) -> String {
    let escs = [('"', "&quot;"), ('\'', "&apos;"), ('<', "&lt;"), ('>', "&gt;"), ('&', "&amp;")];
    let quote_match: Option<&(char, &str)> = escs.iter()
        .find(|pair| {
            let &(e, _) = *pair;
            e == *ch
        });
    let quoted: String = match quote_match {
        Some(&(_, quoted)) => String::from(quoted),
        None => {
            let mut s = String::new();
            s.push(*ch);
            s
        }
    };
    quoted

}


#[test]
fn test_body(){
    let meme = "( ^o^)ノ";
    println!("{}", meme);
    let bodies = parse_memes(meme);
    for b in &bodies{
        println!("{:#?}",b);
    }
    assert_eq!(1, bodies.len());
}

#[test]
fn test_body2(){
    let meme = "ヘ( ^o^)ノ ＼(^_^ )Gimme Five";
    println!("{}", meme);
    let bodies = parse_memes(meme);
    for b in &bodies{
        println!("{:#?}",b);
    }
    assert_eq!(2, bodies.len());
}

#[test]
fn test_position(){
    let meme = "ヘ( ^o^)ノ ＼(^_^ )Gimme Five";
    println!("{}", meme);
    let bodies = parse_memes(meme);
    for b in &bodies{
        println!("{:#?}",b);
    }
    assert_eq!(1, bodies.len());
}
