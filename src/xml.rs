use regex::Regex;

use crate::xml;

pub fn filter_and_fix(mut content: String) -> String {

    let content_clone = content.clone();

    let mut output = String::new();
    let mut path_data = Vec::new();

    let filters = [
        r"<defs[\s\S]*?</defs>",
        r#"<g[^>]*>"#, r"</g>",
        r#"transform="[^"]*""#, r#"style="[^"]*""#,
        r#"stroke="[^"]*""#, r#"fill="[^"]*""#,
    ];
    for f in filters {
        let re = Regex::new(f).unwrap();
        content = re.replace_all(&content, "").to_string();
    }

    for cap in Regex::new(r#"<rect\b[^>]*/?>"#).unwrap().find_iter(&content) {
        path_data.push(xml::rect_to_path(cap.as_str()));
    }
    for cap in Regex::new(r#"<circle\b[^>]*/?>"#).unwrap().find_iter(&content) {
        path_data.push(xml::circle_to_path(cap.as_str()));
    }
    for cap in Regex::new(r#"<ellipse\b[^>]*/?>"#).unwrap().find_iter(&content) {
        path_data.push(xml::ellipse_to_path(cap.as_str()));
    }
    for cap in Regex::new(r#"<line\b[^>]*/?>"#).unwrap().find_iter(&content) {
        path_data.push(xml::line_to_path(cap.as_str()));
    }
    for cap in Regex::new(r#"<polygon\b[^>]*/?>"#).unwrap().find_iter(&content) {
        path_data.push(xml::polyline_to_path(cap.as_str(), true));
    }
    for cap in Regex::new(r#"<polyline\b[^>]*/?>"#).unwrap().find_iter(&content) {
        path_data.push(xml::polyline_to_path(cap.as_str(), false));
    }
    for cap in Regex::new(r#"<path\b[^>]+d="[^"]+"[^>]*/?>"#).unwrap().find_iter(&content) {
        path_data.push(cap.as_str().to_string());
    }

    // Объединённый SVG
    if let Some(viewbox) = xml::extract(&content_clone, "viewBox") {
        output = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="{}">
<path d="{}"/>
</svg>"#,
            viewbox,
            path_data
                .iter()
                .filter_map(|p| xml::extract(p, "d"))
                .collect::<Vec<_>>()
                .join(" ")
        );
    }

    return output;
}

pub fn rect_to_path(tag: &str) -> String {
    let x = extract(tag, "x").unwrap_or("0".to_string());
    let y = extract(tag, "y").unwrap_or("0".to_string());
    let w = extract(tag, "width").unwrap_or("0".to_string());
    let h = extract(tag, "height").unwrap_or("0".to_string());
    format!(r#"<path d="M{x},{y} h{w} v{h} h-{w} Z"/>"#)
}

pub fn circle_to_path(tag: &str) -> String {
    let cx = extract(tag, "cx").unwrap_or("0".to_string());
    let cy = extract(tag, "cy").unwrap_or("0".to_string());
    let r = extract(tag, "r").unwrap_or("0".to_string());
    format!(
        r#"<path d="M{cx},{cy} m-{r},0 a{r},{r} 0 1,0 {r2},0 a{r},{r} 0 1,0 -{r2},0"/>"#,
        cx = cx,
        cy = cy,
        r = r,
        r2 = r.parse::<f32>().unwrap_or(0.0) * 2.0
    )
}

pub fn ellipse_to_path(tag: &str) -> String {
    let cx = extract(tag, "cx").unwrap_or("0".to_string());
    let cy = extract(tag, "cy").unwrap_or("0".to_string());
    let rx = extract(tag, "rx").unwrap_or("0".to_string());
    let ry = extract(tag, "ry").unwrap_or("0".to_string());
    format!(
        r#"<path d="M{cx},{cy} m-{rx},0 a{rx},{ry} 0 1,0 {rx2},0 a{rx},{ry} 0 1,0 -{rx2},0"/>"#,
        cx = cx,
        cy = cy,
        rx = rx,
        ry = ry,
        rx2 = rx.parse::<f32>().unwrap_or(0.0) * 2.0
    )
}

pub fn line_to_path(tag: &str) -> String {
    let x1 = extract(tag, "x1").unwrap_or("0".to_string());
    let y1 = extract(tag, "y1").unwrap_or("0".to_string());
    let x2 = extract(tag, "x2").unwrap_or("0".to_string());
    let y2 = extract(tag, "y2").unwrap_or("0".to_string());
    format!(r#"<path d="M{x1},{y1} L{x2},{y2}"/>"#)
}

pub fn polyline_to_path(tag: &str, close: bool) -> String {
    if let Some(points) = extract(tag, "points") {
        let mut parts = points.split_whitespace();
        if let Some(start) = parts.next() {
            let mut d = format!("M{}", start);
            for p in parts {
                d += &format!(" L{}", p);
            }
            if close {
                d += " Z";
            }
            return format!(r#"<path d="{}"/>"#, d);
        }
    }
    "".into()
}

pub fn extract(tag: &str, attr: &str) -> Option<String> {
    let pattern = format!(r#"{attr}="([^"]+)""#);
    Regex::new(&pattern)
        .ok()?
        .captures(tag)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
}