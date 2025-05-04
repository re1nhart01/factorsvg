use regex::Regex;

mod fs;
mod xml;
mod args;

fn main() {
    let input = fs::read_file_as_text("src/heating.svg").expect("Can't read file");
    let mut output = String::new();

    let mut path_data = Vec::new();
    let mut content = input.clone();

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
    if let Some(viewbox) = xml::extract(&input, "viewBox") {
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

    fs::create_file(String::from("fixed_out.svg"), output).expect("Can't write file");
}
