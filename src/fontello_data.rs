use serde::{Deserialize, Serialize};


#[derive(Clone)]
#[derive(Debug, Serialize, Deserialize)]
pub struct FontelloConfig {
    pub name: String,
    pub css_prefix_text: String,
    pub css_use_suffix: bool,
    pub hinting: bool,
    pub units_per_em: u32,
    pub ascent: u32,
    pub glyphs: Vec<Glyph>,
}

#[derive(Clone)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Glyph {
    pub uid: String,
    pub css: String,
    pub code: u32,
    pub src: String,
    pub selected: bool,
    pub svg: SvgData,
    pub search: Vec<String>,
}

#[derive(Clone)]
#[derive(Debug, Serialize, Deserialize)]
pub struct SvgData {
    pub path: String,
    pub width: u32,
}
