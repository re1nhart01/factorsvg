use std::io::Error;
use svgdom::{AttributeValue, Document, Node, NodeData};



fn parse_attrs(node: &Node, key: &str) -> f32 {
        match node.attributes().get(key) {
            Some(attr) => match &attr.value {
                AttributeValue::Length(l) => l.num as f32,
                AttributeValue::String(s) => s.parse::<f32>().unwrap_or(0.0),
                _ => 0.0,
            },
            None => 0.0,
        }
}

pub fn get_attr_string(node: &Node, key: &str) -> Option<String> {
    println!("{} KEY", key);

    if let Some(attr) = node.attributes().get(key) {
        let val = match &attr.value {
            AttributeValue::String(s) => Some(s.clone()),
            AttributeValue::Length(l) => Some(l.num.to_string()),
            AttributeValue::Path(p) => Some(format!("{}", p)),
            _ => None,
        };

        if let Some(ref s) = val {
            println!("VALUE: {}", s);
        } else {
            println!("(type not supported)");
        }

        return val;
    }

    println!("(not found)");
    None
}

// fn parse_rectangle<'a>(node_item: Node, arg: &'a str) ->  &'a str {
//     arg
// }

pub fn parse_rectangle_rounded(node_item: Node) -> String {
    let x = parse_attrs(&node_item, "x");
    let y = parse_attrs(&node_item, "y");
    let w = parse_attrs(&node_item, "width");
    let h = parse_attrs(&node_item, "height");
    let rx = parse_attrs(&node_item, "rx");
    let ry = parse_attrs(&node_item, "ry");

    let rx = if rx == 0.0 { ry } else { rx };
    let ry = if ry == 0.0 { rx } else { ry };

    println!("parse rect {}", x);

    if rx > 0.0 || ry > 0.0 {
        return format!(
                    "M {mx},{y} \
        h {w1} \
        a {rx},{ry} 0 0 1 {rx},{ry} \
        v {h1} \
        a {rx},{ry} 0 0 1 -{rx},{ry} \
        h -{w1} \
        a {rx},{ry} 0 0 1 -{rx},-{ry} \
        v -{h1} \
        a {rx},{ry} 0 0 1 {rx},-{ry} \
        Z",
                    mx = x + rx,
                    y = y,
                    w1 = w - 2.0 * rx,
                    h1 = h - 2.0 * ry,
                    rx = rx,
                    ry = ry
        )
    } else {
        return format!("M {x},{y} h {w} v {h} h -{w} Z")
    }
}

pub fn parse_rectangle(node_item: Node) -> String {
    /**
     * <rect x y width height rx ry>	M x,y h width a rx,ry 0 0 1 rx,ry v height a rx,ry 0 0 1 -rx,ry h -width a rx,ry 0 0 1 -rx,-ry v -height a rx,ry 0 0 1 rx,-ry Z (если rx/ry есть)
или
M x,y h width v height h -width Z (если углы прямые)
     * 
     */

    let x = parse_attrs(&node_item, "x");
    let y = parse_attrs(&node_item, "y");
    let w = parse_attrs(&node_item, "width");
    let h = parse_attrs(&node_item, "height");

     let formatted_string = format!("M {x},{y} h {w} v {h} h -{w} Z");


    return formatted_string;
}


pub fn parse_circle(node_item: Node) -> String {
   //<circle cx cy r>	M cx+r,cy A r,r 0 1,0 cx-r,cy A r,r 0 1,0 cx+r,cy

   let cx = parse_attrs(&node_item, "сx");
   let cy = parse_attrs(&node_item, "сy");
   let r = parse_attrs(&node_item, "r");

   let cx_p_r = cx + r;
   let cx_m_r = cx - r;

   let formatted_string = format!(
            "M {cx_p_r},{cy} \
        A {r},{r} 0 1,0 {cx_m_r},{cy} \
        A {r},{r} 0 1,0 {cx_p_r},{cy}"
    );

   return formatted_string;
}

pub fn parse_ellipse(node_item: Node) -> String {
    //<ellipse cx cy rx ry>	M cx+rx,cy A rx,ry 0 1,0 cx-rx,cy A rx,ry 0 1,0 cx+rx,cy

    let cx = parse_attrs(&node_item, "сx");
    let cy = parse_attrs(&node_item, "сy");
    let rx = parse_attrs(&node_item, "rx");
    let ry = parse_attrs(&node_item, "ry");

    let cx_p_rx = cx + rx;
    let cx_m_rx = cx - rx;
 

    let formatted_string = format!(
        "M {cx_p_rx},{cy} \
    A {rx},{ry} 0 1,0 {cx_m_rx},{cy} \
    A {rx},{ry} 0 1,0 {cx_p_rx},{cy}"
    );
    

    return formatted_string;
}

pub fn parse_line(node_item: Node) -> String {
    //<line x1 y1 x2 y2>	M x1,y1 L x2,y2

    let x1 = parse_attrs(&node_item, "x1");
    let y1 = parse_attrs(&node_item, "y1");
    let x2 = parse_attrs(&node_item, "x2");
    let y2 = parse_attrs(&node_item, "y2");

    let formatted_string = format!("M {x1},{y1} L {x2},{y2}");

    return formatted_string;
}

pub fn parse_polyline(node_item: Node) -> String {
    /**
     * <polyline points="x1,y1 x2,y2 ...">	M x1,y1 L x2,y2 L x3,y3 ...
     * 
     */

    let points = get_attr_string(&node_item, "points").unwrap_or(String::from(""));

    if points.len() <= 1 {
        return String::new();
    }
    
    // "10,20 30,40 50,60" => ["10,20", "30,40", "50,60"]
    let points_vec = points
        .split_whitespace() // безопаснее, чем split(" ")
        .filter(|s| !s.trim().is_empty())
        .collect::<Vec<_>>();
    
    let d = points_vec
        .iter()
        .enumerate()
        .filter_map(|(i, point)| {
            let parts = point.split(',').collect::<Vec<_>>();
            if parts.len() != 2 {
                return None; // пропустить некорректную точку
            }
            let x = parts[0].trim();
            let y = parts[1].trim();
            Some(if i == 0 {
                format!("M {x},{y}")
            } else {
                format!("L {x},{y}")
            })
        })
        .collect::<Vec<_>>()
        .join(" ");
    
    return d;
}

pub fn parse_polygon(node_item: Node) -> String {
    /**
     * <polygon points="x1,y1 x2,y2 ...">	M x1,y1 L x2,y2 L x3,y3 ... Z
     * 
     */

     let d_from_polyline = parse_polyline(node_item);

    if d_from_polyline.is_empty() {
        return String::new();
    }

    return format!("{d_from_polyline} Z")
}