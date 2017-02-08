extern crate unicode_width;
extern crate svg;


use unicode_width::UnicodeWidthStr;
use unicode_width::UnicodeWidthChar;
use svg::node::element::Circle as SvgCircle;
use svg::node::element::Text as SvgText;
use svg::Node;
use svg::node::element::SVG;
use svg::node::element::Style;
use svg::node::Text as TextNode;


struct Settings {
    text_width: f32,
    text_height: f32,
}

impl Settings{
    
    fn offset(&self)->(f32, f32){
        //(self.text_width * 1.0, self.text_height * 2.0)
        (0.0, 0.0)
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


#[derive(Debug)]
struct Body{
    memes: Vec<Meme>,
    rest_str: Vec<(usize, String)>
}

impl Body {


    fn has_memes(&self) -> bool{
        !self.memes.is_empty()
    }
    
    fn get_svg_elements(&self, y: usize, settings: &Settings) -> Vec<Box<Node>>{
        let mut svg:Vec<Box<Node>> = vec![];
        /*
        for &(startx, ref text)  in &self.rest_str{
            let svg_text = to_svg_text(text, startx, y, settings, Anchor::Start);    
            svg.push(Box::new(svg_text));
        }
        */
        for meme in &self.memes{
            svg.extend(meme.get_svg_elements(y, settings));
        }
        svg
    }

    // build the rest text in 1 string
    fn unify_rest_text(&self) -> String{
        let mut unify = String::new();
        for &(sx, ref word) in &self.rest_str{
            let lacks  = sx - unify.width();
            if lacks > 0{
                for i in 0..lacks{
                    unify.push(' ')
                }
            }
            unify.push_str(word);
        } 
        unify
    }
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
    
    fn get_svg_elements(&self, y: usize, settings: &Settings) -> Vec<Box<Node>>{
        let mut elements:Vec<Box<Node>> = vec![];
        let left_text = to_svg_text(&self.left_side, self.head.startx, y, settings, Anchor::End);
        elements.push(Box::new(left_text));
        elements.extend(self.head.get_svg_elements(y, settings));
        let right_text = to_svg_text(&self.right_side, self.head.endx, y, settings, Anchor::Start);
        elements.push(Box::new(right_text));
        elements
    }

    
}

fn to_svg_text(s: &str, x: usize, y: usize, settings: &Settings, anchor: Anchor) -> SvgText {
    let px = x as f32 * settings.text_width;
    let py = y as f32 * settings.text_height;
    to_svg_text_pixel(s, px, py, settings, anchor)
}

fn to_svg_text_pixel(s: &str, x: f32, y: f32, settings: &Settings, anchor: Anchor) -> SvgText {
    to_svg_text_pixel_escaped(&escape_str(s), x, y, settings, anchor)
}

fn to_svg_text_pixel_escaped(s: &str, x: f32, y: f32, settings: &Settings, anchor: Anchor) -> SvgText {
    let (offsetx, offsety) = settings.offset();
    let sx = x + offsetx;
    let sy = y + settings.text_height * 3.0 / 4.0 + offsety;
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

    let text_node = TextNode::new(s);
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

    fn distance(&self) -> usize {
        self.endx - self.startx     
    }

    fn get_svg_elements(&self, y: usize, settings:&Settings) -> Vec<Box<Node>> {
        let mut elements: Vec<Box<Node>> = vec![];
        elements.push(Box::new(self.get_circle(y, settings)));
        elements.push(Box::new(self.get_face_text(y, settings)));
        elements
    }

    fn get_face_text(&self, y:usize, settings: &Settings) -> SvgText{
        let c = self.calc_circle(y, settings);
        let sy = y as f32 * settings.text_height;
        let face = format!("<tspan class='head'>(</tspan>{}<tspan class='head'>)</tspan>", escape_str(&self.face));
        to_svg_text_pixel_escaped(&face, c.cx, sy, settings, Anchor::Middle)
    }

    fn calc_circle(&self, y:usize, settings: &Settings) -> Circle {
        let text_width = settings.text_width;
        let text_height = settings.text_height;
        let radius = self.distance() as f32 / 2.0;
        let center = self. startx as f32 + radius;
        let cx = center * text_width; 
        let cy = y as f32 * text_height + text_height / 2.0;
        let cr = radius * text_width;
        Circle{
            cx: cx,
            cy: cy,
            r: cr
        }
    }

    fn get_circle(&self, y: usize, settings: &Settings)-> SvgCircle{
        let c = self.calc_circle(y, settings);
        let (offsetx, offsety) = settings.offset();
        SvgCircle::new()
            .set("cx",c.cx + offsetx)
            .set("cy", c.cy + offsety)
            .set("r", c.r)
    }

}

#[derive(Debug)]
struct Circle{
    cx: f32,
    cy: f32,
    r: f32,
}


/// detect whether the series of string could be a meme
/// has at least 1 full width character (width = 2)
/// has at least 1 zero sized width character (width = 0)
/// has at least 1 character that has more than 1 byte in size
/// unicode value is way up high
fn is_meme(ch: &str) -> bool{
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
    total_width <= 10 && // must be at most 10 character face
    (gte_bytes2 > 0 || gte_width2 > 0
    || zero_width > 0 || gte_unicode_1k > 0
    || total_bytes > total_width
    || !is_expression(ch)
    )
}


fn calc_dimension(s: &str) -> (usize, usize) {
    let mut longest = 0;
    for line in s.lines(){
        let line_width = line.width();
        if line_width > longest{
            longest = line_width
        }
    }
    let line_count = s.lines().count();
    (longest, line_count)
}

/// return an SVG document base from the text infor string
pub fn to_svg(s: &str, text_width: f32, text_height: f32) -> SVG {
    let settings = &Settings{
                text_width: text_width,
                text_height: text_height,
            };
    let mut svg = SVG::new()
            .set("font-size", 14)
            .set("font-family", "arial");

        svg.append(get_styles());
    
    let nodes = to_svg_lines(s,settings);
    for elm in nodes{
        let text_node = TextNode::new(elm.to_string());
        svg.append(text_node);
    }

    let (offsetx, offsety) = settings.offset();
    let (wide, high) = calc_dimension(s);
    let width = wide as f32 * text_width + offsetx;
    let height = (high + 2 ) as f32 * text_height + offsety;
    svg.assign("width", width);
    svg.assign("height", height);
    svg
}


fn get_styles() -> Style {
    let style = r#"
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
    tspan.head{
        fill: none;
        stroke: none;
    }
    "#;
    Style::new(style)
}

/// process and parses each line
fn to_svg_lines(s: &str, settings: &Settings) -> Vec<Box<Node>> {
    let mut elements = vec![];
    let mut y = 0;
    for line in s.lines(){
        let line_elm = get_svg_elements(y, line, settings);
        elements.extend(line_elm);
        y += 1;
    }
    elements
}

/// process only 1 line
fn get_svg_elements(y: usize, s: &str, settings: &Settings) -> Vec<Box<Node>> {
    let body = parse_memes(s);
    body.get_svg_elements(y, &settings)
}

/// return the SVG nodes per line and all the assembled rest of the string that is not a part of the memes
pub fn get_meme_svg(input: &str, text_width: f32, text_height: f32) -> (Vec<Box<Node>>, String) {
    let settings = &Settings{
                text_width: text_width,
                text_height: text_height,
            };
    let mut svg_elements:Vec<Box<Node + 'static>> = vec![];
    let mut relines = String::new();
    let text_width = settings.text_width;
    let text_height = settings.text_height;
    let mut y = 0;
    for line in input.lines(){
        match  line_to_svg_with_excess_str(y, line, settings){
            Some((svg_elm, rest_text)) => {
                relines.push_str(&rest_text);
                relines.push('\n');
                svg_elements.extend(svg_elm);
            },
            None => {
                relines.push_str(line);
                relines.push('\n');
            }
        }
        y += 1;
    } 
    (svg_elements, relines)
}

/// parse the memes and return the svg together with the unmatched strings
fn line_to_svg_with_excess_str(y: usize, s: &str, settings:&Settings) -> Option<(Vec<Box<Node>>, String)>{
    let body = parse_memes(s);
    if body.has_memes(){
        let nodes = body.get_svg_elements(y, settings);
        Some((nodes, body.unify_rest_text()))
    }else{
        None
    }
}

#[test]
fn test_1line(){
    let meme = "";
    let nodes = get_svg_elements(0, meme, &Settings::default());
    assert_eq!(nodes.len(), 0);
}


/// TODO: include parsing the rest of the unused text
fn parse_memes(s: &str) -> Body{
    let mut memes = vec![];
    let mut paren_opened = false;
    let mut meme_face = String::new();
    let mut index = 0;
    let mut total_width = 0;
    let mut face_markers:Vec<Head> = vec![];
    let mut startx = 0;
    let mut start_position = 0;
    let mut meme_start = 0;
    let mut meme_body = String::new();
    let mut meme_left_side = String::new();
    let mut meme_right_side = String::new();
    let mut meme_head = None;
    let total_chars = s.chars().count();
    let mut rest_text:Vec<(usize, String)> = vec![];
    for ch in s.chars(){
        let last_char = index == total_chars - 1;
        if meme_head.is_some(){
            meme_right_side.push(ch);
        }
        if paren_opened && ch == ')'{ //if paren_opened and encountered a closing
            paren_opened  = false;
            if is_meme(&meme_face){
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

        if meme_head.is_none() && (ch == ' ' || last_char){
            meme_start = index + 1;
            if !paren_opened{
                let mut rest_word = meme_body.clone();
                let rest_start = total_width - rest_word.width();
                if last_char{
                   rest_word.push(ch);
                   rest_word.push_str(&meme_face);//the head is unmatched
                }
                rest_text.push((rest_start, rest_word));
            }
            meme_body.clear();
        }
        if meme_head.is_some() && (ch == ' ' || last_char){
            let meme = Meme{
               start_position: meme_start,  
               head: meme_head.clone().unwrap(),
               end_position: index,
               left_side: meme_left_side.clone(), 
               right_side: meme_right_side.clone(),
            };
            memes.push(meme);
            meme_right_side.clear();
            meme_left_side.clear();
            meme_body.clear();
            meme_head = None;
        }
        meme_body.push(ch);
        if let Some(uw) = ch.width(){
            total_width += uw;
        }
        index += 1;
    } 
    Body{
        memes: memes,
        rest_str: regroup_rest_text(&rest_text)
    }
}

fn regroup_rest_text(rest_text: &Vec<(usize, String)>)->Vec<(usize, String)>{
    let mut new_group = vec![];
    //println!("regrouping text..");
    for &(start,ref rest) in rest_text{
        if new_group.is_empty(){
            new_group.push((start, rest.clone()));
        }else{
            if let Some((lastx, last_rest)) = new_group.pop(){
               if lastx + last_rest.width() == start{
                    let mut merged = String::new();
                    merged.push_str(&last_rest);
                    merged.push_str(rest);
                    new_group.push((lastx, merged));
               }else{
                   new_group.push((lastx, last_rest));
                   new_group.push((start, rest.clone()));
               } 
            }
        }
    }
    //println!("new_group: {:#?}", new_group);
    new_group
}

#[test]
fn test_meme() {
    assert!(is_meme(" ͡° ͜ʖ ͡°"));
    assert!(is_meme("⌐■_■"));
    assert!(is_meme("ツ"));
    assert!(!is_meme("svgbobg"));
    assert!(!is_meme("not a meme in space"));
    assert!(is_meme(" -_- "));
    assert!(is_meme("-_-"));
    assert!(!is_meme("     "));
}

#[test]
fn test_expression(){
    assert!(is_meme("^_^"));
    assert!(is_meme("x_x"));
    assert!(!is_meme("+"));
    assert!(!is_meme("x+y"));
    assert!(!is_meme("x^2*y^2"));
    assert!(!is_meme("x^2 * y^2"));
}

#[test]
fn test_bound(){
    let meme = "(♥_♥)";
    let memes = parse_memes(meme); 
    for m in memes.memes{
        let b = m.head;
        println!("bound {:?} d:{} ", b , b.distance());
        assert_eq!(4, b.distance());
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


fn is_operator(c: char) -> bool{
    c == '+' || c == '-' || c == '*' || c == '/'
    || c == '^' || c == '%' || c == '!' || c == ','
    || c == '.' || c == '=' || c == '|' || c == '&'
}

#[test]
fn test_operator(){
    assert!(!is_expression("^_^"));
    assert!(is_operator('+'));
    assert!(is_expression("+"));
    assert!(is_expression("x+y"));
    assert!(is_expression("x^2*y^2"));
    assert!(is_expression("x^2 * y^2"));
}

//TODO: alternate alphanumeric_space and operator
fn is_expression(ch: &str) -> bool{
    is_alphanumeric_space_operator(ch) 
}

fn is_alphanumeric_space_operator(ch:&str) -> bool{
    ch.chars().all(|c| c.is_alphanumeric() || c == ' ' || c == '_' || is_operator(c))
}


#[test]
fn test_body(){
    let meme = "( ^_^)ノ";
    println!("{}", meme);
    let bodies = parse_memes(meme);
    for b in &bodies.memes{
        println!("{:#?}",b);
    }
    assert_eq!(1, bodies.memes.len());
}

#[test]
fn test_body2(){
    let meme = "ヘ( ^_^)ノ ＼(^_^ )Gimme Five";
    println!("{}", meme);
    let bodies = parse_memes(meme);
    for b in &bodies.memes{
        println!("{:#?}",b);
    }
    assert_eq!(2, bodies.memes.len());
}

#[test]
fn test_rest_of_text(){
    let meme = r#"The rest of   凸(•̀_•́)凸❤️ ( ͡° ͜ʖ ͡°) \(°□°)/层∀  the text is here"#;
    println!("{}", meme);
    let bodies = parse_memes(meme);
    println!("{:#?}",bodies);
    assert_eq!(3, bodies.memes.len());
    assert_eq!(2, bodies.rest_str.len());
}

#[test]
fn test_unify_rest_of_text(){
    let meme = r#"The rest of   凸(•̀_•́)凸❤️ ( ͡° ͜ʖ ͡°) \(°□°)/层∀  the text is here"#;
    let resi = r#"The rest of                                   the text is here"#;
    println!("{}", meme);
    let bodies = parse_memes(meme);
    println!("{:#?}",bodies);
    assert_eq!(3, bodies.memes.len());
    assert_eq!(2, bodies.rest_str.len());
    assert_eq!(meme.width(), bodies.unify_rest_text().width());
    println!("residue: {} meme: {} rest_text:{}", resi.width(), meme.width(), bodies.unify_rest_text().width());
    assert_eq!(resi.to_string(), bodies.unify_rest_text());
}

#[test]
fn test_meme_equation(){
    let meme= r#"Equations are not rendered? ( -_- )  __(x+y)__  (^_^) (x^2+y^2)x"#;
    println!("{}", meme);
    let bodies = parse_memes(meme);
    println!("{:#?}",bodies);
    assert_eq!(2, bodies.memes.len());
    assert_eq!(3, bodies.rest_str.len());
}

#[test]
fn test_meme_equation2(){
    let meme= r#"( -_- ) __(x+y)__ (^_^) (x^2+y^2)"#;
    println!("{}", meme);
    let bodies = parse_memes(meme);
    println!("{:#?}",bodies);
    assert_eq!(2, bodies.memes.len());
    assert_eq!(2, bodies.rest_str.len());
}


#[test]
fn test_meme_unmatched_face(){
    let meme= r#"(╯°□°] ╯︵ ┬─┻"#;
    println!("{}", meme);
    let bodies = parse_memes(meme);
    println!("{:#?}",bodies);
    assert_eq!(0, bodies.memes.len());
    assert_eq!(1, bodies.rest_str.len());
}


