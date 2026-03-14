use gloo::utils::document;
use js_sys::{Array, Math};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::Element;

#[wasm_bindgen]
#[derive(Clone)]
pub struct IWordInfo {
    #[wasm_bindgen(getter_with_clone)]
    pub text: Vec<String>,
    #[wasm_bindgen(getter_with_clone)]
    pub widths: Vec<f64>,
    pub font_size: i32,
    pub ellipsis: bool,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct LineClampProps {
    #[wasm_bindgen(getter_with_clone)]
    pub texts: Vec<String>,
    pub content_width: i32,
    pub content_height: i32,
}
#[wasm_bindgen]
pub struct InputResultForEllipsis {
    #[wasm_bindgen(getter_with_clone)]
    pub html: Vec<String>,
    pub font_size: i32,
}

#[wasm_bindgen]
pub struct LineClamp {
    #[wasm_bindgen(getter_with_clone)]
    pub element: Element,
    pub step: i32,
    pub min_font_size: i32,
    pub max_font_size: i32,
    pub line_height: i32,
    pub min_width_height: i32,
    #[wasm_bindgen(getter_with_clone)]
    pub ellipsis: String,
    #[wasm_bindgen(getter_with_clone)]
    pub line_clamp_props: LineClampProps,
}

#[wasm_bindgen]
impl LineClamp {
    #[wasm_bindgen(constructor)]
    pub fn new(line_clamp_props: LineClampProps) -> LineClamp {
        LineClamp {
            element: document().create_element("div").unwrap(),
            step: 1,
            min_font_size: 10,
            max_font_size: 14,
            line_height: 14,
            min_width_height: 5,
            ellipsis: String::from("..."),
            line_clamp_props,
        }
    }

    pub fn calc(&self) -> InputResultForEllipsis {
        if self.line_clamp_props.content_height <= self.min_width_height
            || self.line_clamp_props.content_width <= self.min_width_height
        {
            return self.ellipsis_result().unwrap();
        }

        let words_info = self.get_word_widths(self.max_font_size, vec![]);
        let can_widths = self.calc_word_width_can_in_content(
            &words_info,
            self.line_clamp_props.content_width as f64,
        );
        let max_line = (self.line_clamp_props.content_height / self.line_height) as usize;

        if max_line == 0 {
            return self.ellipsis_result().unwrap();
        }

        let can_heights = if can_widths.length() > max_line as u32 {
            let mut new_arr = Array::new();
            for i in 0..(max_line as u32) {
                new_arr.push(&can_widths.get(i));
            }
            new_arr
        } else {
            can_widths.clone()
        };

        if can_heights.length() == 0 {
            return self.ellipsis_result().unwrap();
        }

        if words_info.ellipsis || can_heights.length() != can_widths.length() {
            let ellipsis_width = self.get_width_of_content(
                self.ellipsis.clone(),
                words_info.font_size,
            ) as f64;
            let length = if max_line > can_heights.length() as usize {
                can_heights.length() as usize - 1
            } else {
                max_line - 1
            };

            let last_line_js = can_heights.get(length as u32);
            let last_line_arr = last_line_js.dyn_into::<Array>().unwrap();
            let last_line_heights: Vec<f64> = last_line_arr.iter().map(|v| v.as_f64().unwrap()).collect();

            let mut last_words = Vec::new();
            let mut last_width = 0.0;

            for &width in &last_line_heights {
                last_width += width;
                if last_width + ellipsis_width > self.line_clamp_props.content_width as f64 {
                    last_words.push(-1.0);
                    break;
                } else {
                    last_words.push(width);
                }
            }

            if last_words.last() != Some(&-1.0) {
                last_words.push(-1.0);
            }

            let modified_arr = Array::new();
            for i in 0..can_heights.length() {
                if i == length as u32 {
                    let arr = Array::new();
                    for &w in &last_words {
                        arr.push(&JsValue::from_f64(w));
                    }
                    modified_arr.push(&arr);
                } else {
                    modified_arr.push(&can_heights.get(i));
                }
            }

            let mut i = 0;
            let mut html = Vec::new();

            for j in 0..modified_arr.length() {
                let item_js = modified_arr.get(j);
                let item_arr = item_js.dyn_into::<Array>().unwrap();
                let item: Vec<f64> = item_arr.iter().map(|v| v.as_f64().unwrap()).collect();
                let mut str = String::new();
                for &t in &item {
                    if t == -1.0 {
                        str = format!("{}{}", str.trim(), self.ellipsis);
                    } else {
                        str = format!("{} {} ", str, self.line_clamp_props.texts[i]);
                        i += 1;
                    }
                }
                html.push(self.transaction_to_html(&str));
            }

            InputResultForEllipsis {
                html,
                font_size: words_info.font_size,
            }
        } else {
            let mut i = 0;
            let mut html = Vec::new();

            for j in 0..can_heights.length() {
                let item_js = can_heights.get(j);
                let item_arr = item_js.dyn_into::<Array>().unwrap();
                let item: Vec<f64> = item_arr.iter().map(|v| v.as_f64().unwrap()).collect();
                let mut str = String::new();
                for _ in &item {
                    str = format!("{} {} ", str, self.line_clamp_props.texts[i]);
                    i += 1;
                }
                html.push(self.transaction_to_html(&str));
            }

            InputResultForEllipsis {
                html,
                font_size: words_info.font_size,
            }
        }
    }

    pub fn transaction_to_html(&self, value: &str) -> String {
        let format_html = format!(
            "<span style='display:inline-block;'>{}</span>",
            value.trim()
        );
        format_html
    }

    pub fn ellipsis_result(&self) -> Result<InputResultForEllipsis, String> {
        let mut html_result = Vec::new();
        let html: String = self.transaction_to_html(&self.ellipsis);
        html_result.push(html);
        Ok(InputResultForEllipsis {
            html: html_result,
            font_size: self.min_font_size,
        })
    }

    pub fn get_word_widths(&self, font_size: i32, incomplete_width: Vec<i32>) -> IWordInfo {
        if font_size < self.min_font_size {
            return IWordInfo {
                text: self.line_clamp_props.texts[0..incomplete_width.len()].to_vec(),
                ellipsis: true,
                widths: incomplete_width.into_iter().map(|w| w as f64).collect(),
                font_size: self.min_font_size,
            };
        } else {
            let mut widths: Vec<f64> = self
                .line_clamp_props
                .texts
                .iter()
                .map(|t| self.get_width_of_content(t.clone(), font_size) as f64)
                .collect();
            let space_width = self.calc_space_width(font_size);
            let width_len = widths.len() - 1;
            widths = widths
                .into_iter()
                .enumerate()
                .map(|(index, w)| {
                    if width_len != index {
                        w + space_width as f64
                    } else {
                        w
                    }
                })
                .collect();
            if widths
                .iter()
                .all(|&w| w <= self.line_clamp_props.content_width as f64)
            {
                return IWordInfo {
                    text: self.line_clamp_props.texts.clone(),
                    widths,
                    font_size: font_size,
                    ellipsis: false,
                };
            }

            let mut new_incomplete = Vec::new();
            for &w in &widths {
                if w <= self.line_clamp_props.content_width as f64 {
                    new_incomplete.push(w);
                } else {
                    break;
                }
            }
 
            self.get_word_widths(font_size - self.step, new_incomplete.into_iter().map(|w| w as i32).collect())
        }
    }

    pub fn get_width_of_content(&self, content: String, font_size: i32) -> i32 {
        let div_element = Rc::new(RefCell::new(self.element.clone()));
        let div_element_add_attr = div_element.clone();
        let div_element_set_inner = div_element.clone();
        let div_element_move = div_element.clone();
        let format_for_style = format!(
            "display:inline-block;font-size:{}px;line-height:{}px",
            font_size, self.line_height
        );
        div_element_add_attr
            .borrow_mut()
            .set_attribute("style", &format_for_style).unwrap();
        div_element_set_inner.borrow_mut().set_inner_html(&content);
        let body_tags = document().body().unwrap();
        body_tags.append_child(&div_element_move.borrow()).unwrap();
        let offset_width = <Element as Clone>::clone(&div_element_move.borrow())
            .dyn_into::<web_sys::HtmlElement>()
            .ok()
            .unwrap()
            .offset_width();
        body_tags.remove_child(&div_element_move.borrow()).unwrap();
        offset_width
    }

    pub fn calc_space_width(&self, font_size: i32) -> i32 {
        let width = self.get_width_of_content(String::from("space"), font_size);
        let word_width = self.get_width_of_content(String::from("pace"), font_size);
        let result = width - word_width;
        result
    }

    pub fn calc_word_width_can_in_content(&self, result: &IWordInfo, content_width: f64) -> Array {
        let widths = result.widths.clone();
        let mut groups: Vec<Vec<f64>> = Vec::new();
        let mut current_group: Vec<f64> = Vec::new();
        let mut current_sum: f64 = 0.0;

        for &width in &widths {
            if current_sum + width <= content_width {
                current_group.push(width);
                current_sum += width;
            } else {
                if current_group.len() > 0 {
                    groups.push(current_group);
                }
                current_group = [width].to_vec();
                current_sum = width;
            }
        }

        if current_group.len() > 0 {
            groups.push(current_group);
        }

        let result_array = Array::new();
        for group in groups {
            let arr = Array::new();
            for &num in &group {
                arr.push(&JsValue::from_f64(num));
            }
            result_array.push(&arr);
        }
        result_array
    }
}
