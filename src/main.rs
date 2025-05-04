use std::{fs::File, ptr::null};

use regex::Regex;
use svgdom::{Document, ElementId};

mod xml;    
mod fs;


fn main() {
    let mut file = fs::read_file_as_text("src/no-internet.svg").unwrap_or_default();

    let re = Regex::new(r#"<g\b[^>]*>"#).unwrap();
    file = re.replace_all(file.clone().as_str(), "").to_string();
    //.replace("\n", "") 
    file = file.replace("<g>", "").replace("</g>", "");
    println!("{}", file);
    
    let as_str = file.as_str();

    let mut document = Document::from_str(as_str).unwrap();

    let mut all_path: String = String::new();

    for node in document.root().descendants() {
        if let Some(tag_id) = node.tag_id() {
            println!("{}", tag_id);
            let d = match tag_id {
                ElementId::Rect => xml::parse_rectangle_rounded(node.clone()),
                ElementId::Circle => xml::parse_circle(node.clone()),
                ElementId::Ellipse => xml::parse_ellipse(node.clone()),
                ElementId::Line => xml::parse_line(node.clone()),
                ElementId::Polyline => xml::parse_polyline(node.clone()),
                ElementId::Polygon => xml::parse_polygon(node.clone()),
                ElementId::Path => {
                    xml::get_attr_string(&node, "d").unwrap_or_default()
                }
                _ => String::new(),
            };

            println!("Test {} {}", tag_id, d);
    
            if !d.is_empty() {
                all_path.push_str(&format!("{d} "));
            }
        }

        //let x = node.get_attribute("x").map(|v| v.value.parse::<f32>().unwrap_or(0.0)).unwrap_or(0.0);
    }

    println!("{}", all_path);

    let final_svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1000 1000">
    <path d="{all_path}"/>
    </svg>"#);

    let is_created = fs::create_file(String::from("aboba_cav.svg"), final_svg);

    match is_created {
        Some(error) => println!("Error: {}", error),
        None => println!("Create successfully")
    }

}
