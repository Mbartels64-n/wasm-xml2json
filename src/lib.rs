use wasm_bindgen::prelude::*;
use quick_xml::reader::Reader;
use quick_xml::events::Event;

#[wasm_bindgen]
pub fn parse_string_promise(xml: &str) -> Result<JsValue, JsValue> {
    let mut reader = Reader::from_str(xml);
    let mut stack = Vec::new();
    let mut current_object = js_sys::Object::new();
    let mut root_object = None;

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                current_object = process_start_event(&e, &mut stack, current_object, &mut root_object)?;
            }
            Ok(Event::End(_)) => {
                if let Some(parent) = stack.pop() {
                    current_object = parent;
                }
            }
            Ok(Event::Text(e)) => {
                process_text_event(e, &mut current_object)?;
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(JsValue::from_str(&format!("Error parsing XML: {:?}", e))),
            _ => {}
        }
    }

    Ok(root_object.unwrap_or_else(|| js_sys::Object::new()).into())
}

fn process_start_event(
    e: &quick_xml::events::BytesStart,
    stack: &mut Vec<js_sys::Object>,
    mut current_object: js_sys::Object,
    root_object: &mut Option<js_sys::Object>,
) -> Result<js_sys::Object, JsValue> {
    let name = std::str::from_utf8(e.name().as_ref())
        .map_err(|e| JsValue::from_str(&format!("Error parsing tag name: {:?}", e)))?
        .to_string();

    stack.push(current_object.clone());
    current_object = js_sys::Object::new();

    handle_attributes(e, &mut current_object)?;

    if root_object.is_none() {
        *root_object = Some(js_sys::Object::new());
        js_sys::Reflect::set(root_object.as_ref().unwrap(), &JsValue::from_str(&name), &current_object).unwrap();
    } else {
        let parent_index = stack.len() - 1;

        let existing = js_sys::Reflect::get(&stack[parent_index], &JsValue::from_str(&name)).unwrap();

        if existing.is_undefined() {
            js_sys::Reflect::set(&stack[parent_index], &JsValue::from_str(&name), &current_object).unwrap();
        } else if existing.is_array() {
            js_sys::Array::from(&existing).push(&current_object);
        } else {
            let array = js_sys::Array::new();
            array.push(&existing);
            array.push(&current_object);
            js_sys::Reflect::set(&stack[parent_index], &JsValue::from_str(&name), &array).unwrap();
        }
    }

    Ok(current_object)
}

fn process_text_event(e: quick_xml::events::BytesText, current_object: &mut js_sys::Object) -> Result<(), JsValue> {
    let text = e.unescape()
        .map_err(|e| JsValue::from_str(&format!("Error parsing text: {:?}", e)))?
        .to_string();
    if !text.trim().is_empty() {
        js_sys::Reflect::set(current_object, &JsValue::from_str("#text"), &JsValue::from_str(&text)).unwrap();
    }
    Ok(())
}

fn handle_attributes(e: &quick_xml::events::BytesStart, current_object: &mut js_sys::Object) -> Result<(), JsValue> {
    for attr in e.attributes().flatten() {
        let key = std::str::from_utf8(attr.key.as_ref())
            .map_err(|e| JsValue::from_str(&format!("Error parsing attribute key: {:?}", e)))?
            .to_string();
        let value = attr.unescape_value()
            .map_err(|e| JsValue::from_str(&format!("Error parsing attribute value: {:?}", e)))?
            .to_string();
        js_sys::Reflect::set(current_object, &JsValue::from_str(&key), &JsValue::from_str(&value)).unwrap();
    }
    Ok(())
}
