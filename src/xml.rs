use regex::Regex;

pub fn filter_and_fix(mut content: String) -> (String, String, String) {
    let content_clone = content.clone();

    let mut output = String::new();
    let mut path_data = Vec::new();

    let filters = [
        r"<defs[\s\S]*?</defs>",
        r#"<g[^>]*>"#,
        r"</g>",
        r#"transform="[^"]*""#,
        r#"style="[^"]*""#,
        r#"stroke="[^"]*""#,
        r#"fill="[^"]*""#,
    ];
    for f in filters {
        let re = Regex::new(f).unwrap();
        content = re.replace_all(&content, "").to_string();
    }

    for cap in Regex::new(r#"<rect\b[^>]*/?>"#)
        .unwrap()
        .find_iter(&content)
    {
        path_data.push(rect_to_path(cap.as_str()));
    }
    for cap in Regex::new(r#"<circle\b[^>]*/?>"#)
        .unwrap()
        .find_iter(&content)
    {
        path_data.push(circle_to_path(cap.as_str()));
    }
    for cap in Regex::new(r#"<ellipse\b[^>]*/?>"#)
        .unwrap()
        .find_iter(&content)
    {
        path_data.push(ellipse_to_path(cap.as_str()));
    }
    for cap in Regex::new(r#"<line\b[^>]*/?>"#)
        .unwrap()
        .find_iter(&content)
    {
        path_data.push(line_to_path(cap.as_str()));
    }
    for cap in Regex::new(r#"<polygon\b[^>]*/?>"#)
        .unwrap()
        .find_iter(&content)
    {
        path_data.push(polyline_to_path(cap.as_str(), true));
    }
    for cap in Regex::new(r#"<polyline\b[^>]*/?>"#)
        .unwrap()
        .find_iter(&content)
    {
        path_data.push(polyline_to_path(cap.as_str(), false));
    }
    for cap in Regex::new(r#"<path\b[^>]+d="[^"]+"[^>]*/?>"#)
        .unwrap()
        .find_iter(&content)
    {
        path_data.push(cap.as_str().to_string());
    }


    let svg_d = path_data
    .iter()
    .filter_map(|p| extract(p, "d"))
    .collect::<Vec<_>>()
    .join(" ");

    if let Some(viewbox) = extract(&content_clone, "viewBox") {
        output = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="{}">
<path d="{}"/>
</svg>"#,
            viewbox,
            svg_d            
        );
        return (output, svg_d, viewbox);
    }

    return (String::new(), String::new(), String::new())
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

pub fn normalize_svg_path_full(path: &str, current_width: f64, desired_width: f64) -> String {
    let arc_fixed: String = fix_arc_flags(path);
    let scaled = rescale_svg_path(&arc_fixed, current_width, desired_width);
    normalize_format(&scaled)
}

/// Фиксит сломанные команды дуги (например, `a95.9,95.9 0.0 50.0,0.0 191.8,0.0`)
fn fix_arc_flags(path: &str) -> String {
    let arc_re = Regex::new(r"(?P<pre>a[^\d]+[\d.]+,[\d.]+\s+[\d.]+\s+)50\.0,0\.0").unwrap();
    arc_re.replace_all(path, "${pre}1,1").to_string()
}

/// Масштабирует все числа в path
fn rescale_svg_path(path: &str, current_width: f64, desired_width: f64) -> String {
    let scale = desired_width / current_width;
    let number_re = Regex::new(r"-?\d+(\.\d+)?").unwrap();

    number_re
        .replace_all(path, |caps: &regex::Captures| {
            let num: f64 = caps[0].parse().unwrap_or(0.0);
            format!("{:.1}", num * scale)
        })
        .to_string()
}

/// Форматирует svg-путь:
/// - убирает `.0` → `476.0` → `476`
/// - заменяет `,` на пробел
/// - убирает лишние пробелы
fn normalize_format(s: &str) -> String {
    let remove_zeroes = Regex::new(r"(\d+)\.0\b").unwrap();
    let collapse_spaces = Regex::new(r"\s+").unwrap();
    let replaced = s.replace(",", " ");
    let no_trailing = remove_zeroes.replace_all(&replaced, "$num");
    collapse_spaces.replace_all(&no_trailing, " ").trim().to_string()
}

pub fn repair_svg_path(path: &str) -> String {
    let mut output = String::new();
    let mut last_move_to: Option<(f64, f64)> = None;

    // Убираем запятые
    let path = path.replace(",", " ");

    // Разделим всё по пробелам и проанализируем
    let tokens: Vec<&str> = path.split_whitespace().collect();
    let mut i = 0;

    while i < tokens.len() {
        let token = tokens[i];

        match token {
            // относительное m → абсолютное M
            "m" => {
                if i + 2 <= tokens.len() {
                    let dx: f64 = tokens[i + 1].parse().unwrap_or(0.0);
                    let dy: f64 = tokens[i + 2].parse().unwrap_or(0.0);
                    let (x, y) = if let Some((last_x, last_y)) = last_move_to {
                        (last_x + dx, last_y + dy)
                    } else {
                        (dx, dy)
                    };
                    output += &format!("M{:.1} {:.1} ", x, y);
                    last_move_to = Some((x, y));
                    i += 3;
                } else {
                    i += 1;
                }
            }

            // абсолютное M
            "M" => {
                if i + 2 <= tokens.len() {
                    let x: f64 = tokens[i + 1].parse().unwrap_or(0.0);
                    let y: f64 = tokens[i + 2].parse().unwrap_or(0.0);
                    output += &format!("M{:.1} {:.1} ", x, y);
                    last_move_to = Some((x, y));
                    i += 3;
                } else {
                    i += 1;
                }
            }

            // дуга a → A с автозаполнением флагов
            "a" | "A" => {
                if i + 5 <= tokens.len() {
                    let rx = tokens[i + 1];
                    let ry = tokens[i + 2];
                    let rot = tokens[i + 3];

                    // Поддержка: если нет флагов, вставляем 1,1
                    let (large_flag, sweep_flag, dx, dy) = if i + 7 <= tokens.len() {
                        (
                            tokens[i + 4],
                            tokens[i + 5],
                            tokens[i + 6],
                            tokens[i + 7],
                        )
                    } else if i + 5 <= tokens.len() {
                        (
                            "1",
                            "1",
                            tokens[i + 4],
                            tokens[i + 5],
                        )
                    } else {
                        ("1", "1", "0", "0")
                    };

                    output += &format!(
                        "A{} {} {} {} {} {} {} ",
                        rx, ry, rot, large_flag, sweep_flag, dx, dy
                    );

                    i += if i + 7 <= tokens.len() { 8 } else { 6 };
                } else {
                    i += 1;
                }
            }

            // другие команды — просто копируем
            "L" | "H" | "V" | "Z" => {
                output += &format!("{} ", token);
                i += 1;
            }

            // число после команды — просто добавляем
            _ => {
                output += &format!("{} ", token);
                i += 1;
            }
        }
    }

    // Убираем .0 и лишние пробелы
    let remove_zeroes = Regex::new(r"(\d+)\.0\b").unwrap();
    let collapse_spaces = Regex::new(r"\s+").unwrap();
    let clean = remove_zeroes.replace_all(&output, "$1");
    collapse_spaces.replace_all(&clean, " ").trim().to_string()
}
