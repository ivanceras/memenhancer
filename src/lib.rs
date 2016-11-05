extern crate unicode_width;
extern crate svg;


use unicode_width::UnicodeWidthStr;
use unicode_width::UnicodeWidthChar;
use svg::node::element::Circle as SvgCircle;


/// The whole meme body
/// 
#[derive(Clone,Debug)]
struct Meme{
    /// location to the left until a space is encountered
    left: Option<usize>,
    head: Head,
    /// location to the right until a space is encoutered
    right: Option<usize>,
    /// all chacaters to the left and right without whitespace in between
    source: String
}

/// The head of the meme
/// the face is the string in between
/// used in detecting if it's a valid meme or not
#[derive(Clone,Debug)]
struct Head{
    left: Option<usize>,
    face: String,
    right: Option<usize>
}

impl Head{

    fn is_meme_face(&self) -> bool {
        is_meme(&self.face)
    }

    fn distance(&self) -> usize {
        if let Some(right) = self.right{
            if let Some(left) = self.left{
                right - left     
            }else { 0 }
        }else{ 0 }
    }

    fn face_width(&self) -> usize{
        self.face.width()
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

pub fn to_svg(s: &str, x: usize, y:usize, 
        text_width: f32, text_height: f32) -> String {
    let svg_circles = get_svg_circles(s, x, y, text_width, text_height);
    let mut svg = String::new();
    for c in svg_circles{
        svg.push_str(&c.to_string());
    }
    svg
}

pub fn get_svg_circles(s: &str, x: usize, y:usize, 
        text_width: f32, text_height: f32) -> Vec<SvgCircle> {
    let circles = get_circles(s, x, y, text_width, text_height);
    let mut svg_circles = vec![];
    for c in circles{
        svg_circles.push(circle_to_svg(&c));
    }
    svg_circles
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
    let text_width = 8.0;
    let text_height = 16.0;
    let xloc = x as f32 * text_width;
    let yloc = y as f32 * text_height;
    //println!("{}", s);
    let memes = meme_faces(s);
    for m in memes{
        /*
        println!("face: {}", m.face);
        println!(" start: {}", m.left.unwrap());
        println!(" width: {}", m.face_width());
        println!(" distance: {}", m.bound_distance());
        */
        let radius = m.face_width() as f32 / 2.0 + 0.5;
        let left = m.left.unwrap() as f32;
        let right = m.right.unwrap() as f32;
        let left_adjustment = 0.5 + left * 0.1;
        let right_adjustment = 0.5 + right * 0.1;
        let radius_adjustment = radius + radius * 0.1;
        //let center = left + left_adjustment + radius_adjustment ;
        let center = 0.5 + left + radius;
        let comp_center = center * text_width + xloc;
        /*
        println!("center: {}", center * text_width);
        println!("radious: {}", radius * text_width);
        */
        let circle = Circle{
            x: comp_center,
            y: yloc + text_height / 2.0,
            r: radius * text_width
        };
        //println!("{:?}", circle);
        circles.push(circle);
        //println!(" ");
    }
    circles
}

fn meme_faces(s: &str) -> Vec<Head> {
    let marks = mark_faces(s);
    marks.into_iter()
        .filter(|m| m.is_meme_face()).collect()
}


fn mark_faces(s: &str) -> Vec<Head> {
    let mut paren_start = false;
    let mut meme_face = String::new();
    let mut index = 0;
    let mut total_width = 0;
    let mut face_markers:Vec<Head> = vec![];
    for ch in s.chars(){
        if ch == ')'{
            paren_start  = false;
            //println!("meme_face: {}", meme_face);
            let last_bound = face_markers.pop();
            if let Some(last_bound) = last_bound{
                let mut upd_bound = last_bound.clone();   
                upd_bound.face = meme_face.clone();
                upd_bound.right = Some(total_width);
                face_markers.push(upd_bound);
            }
            meme_face.clear();
        }
        if paren_start{
           meme_face.push(ch); 
        }
        if ch == '('{
            paren_start = true;
            meme_face.clear();
            let sbound = Head{
                left: Some(total_width),
                face: "".into(),
                right: None
            };
            face_markers.push(sbound);
        }
        if let Some(uw) = ch.width(){
            total_width += uw;
        }
        index += 1;
    } 
    face_markers
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
    let bounds = meme_faces(meme); 
    for b in bounds{
        println!("bound {:?} d:{} w:{} ", b, b.distance(), b.face_width());
    }
    panic!();
}
