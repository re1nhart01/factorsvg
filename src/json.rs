use crate::{fontello_data, fs_utils};
use std::io::Error;


pub fn read_json_file_fontello(path: String) -> Result<fontello_data::FontelloConfig, Error> {
    let file_data = fs_utils::read_file_as_text(path.as_str())?;

    let instance: fontello_data::FontelloConfig = serde_json::from_str(file_data.as_str())?;

    return Ok(instance);
}

pub fn reload_config(model: fontello_data::FontelloConfig, sortable: String) -> fontello_data::FontelloConfig {

    let mut model_copy = model.clone();
    model_copy.glyphs.sort_by(|a, b| a.code.cmp(&b.code));

    return model_copy;
}



pub fn add_glyph(
    model: fontello_data::FontelloConfig,
    css: String,
    svg: fontello_data::SvgData,
) -> fontello_data::FontelloConfig {

    let uid_md5: String = uuid::Uuid::new_v4().to_string();
    let last_id = if let Some(last) = model.glyphs.last() {
        u32::from(last.code)
    } else {
        1u32
    };

    let css_name = css.replace("-", "_").replace(".svg", "").replace(" ", "_");

    let new_modal = fontello_data::Glyph{
        code: last_id + 1,
        css: css_name.clone(),
        src: String::from("custom_icons"),
        search: vec![css_name.clone()],
        selected: true,
        svg: svg,
        uid: uid_md5,
    };

    let mut model_copy = model.clone();

    let is_already_exists = model.glyphs.iter().any(|elem| {
        css_name == elem.css
    });

    if is_already_exists {
        println!("Item with name {} already exists", css_name);
        return model_copy
    }

    model_copy.glyphs.push(new_modal);

    return model_copy;
}


pub fn save_file(data: fontello_data::FontelloConfig, input: String) -> Option<Error> {
    let string_content = serde_json::to_string(&data).unwrap();

    fs_utils::create_file(input, string_content)
}