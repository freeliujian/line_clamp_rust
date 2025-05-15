use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::Element;
use gloo::utils::{document};
use js_sys::{Array, Math};

#[wasm_bindgen]
#[derive(Clone)]
pub struct IWordInfo {
    #[wasm_bindgen(getter_with_clone)] 
    pub text: Vec<String>,
    #[wasm_bindgen(getter_with_clone)] 
    pub widths: Vec<f64>,
    pub font_size: usize,
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
    pub step: f32,
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
            step: 0.2,
            min_font_size: 10,
            max_font_size: 14,
            line_height: 14,
            min_width_height: 5,
            ellipsis: String::from("..."),
            line_clamp_props
        }
    }

    pub fn calc(&self) -> InputResultForEllipsis {
        if self.line_clamp_props.content_height <= self.min_width_height 
            || self.line_clamp_props.content_width <= self.min_width_height 
        {
            return self.ellipsis_result().unwrap();
        }

        let words_info = self.get_word_widths(self.max_font_size, vec![]);
        let can_widths = self.calc_word_width_can_in_content(&words_info, self.line_clamp_props.content_width as f64);
        let max_line = (self.line_clamp_props.content_height / self.line_height) as usize;

        if max_line == 0 {
            return self.ellipsis_result().unwrap();
        }

        let can_heights = if can_widths.length() > max_line.try_into().unwrap() {
            can_widths.into_iter().take(max_line).collect()
        } else {
            can_widths.clone()
        };

        if can_heights.length() == 0 {
            return self.ellipsis_result().unwrap();
        }

        if words_info.ellipsis || can_heights.length() != can_widths.length() {
            let ellipsis_width = self.get_width_of_content(self.ellipsis.clone(), words_info.font_size.try_into().unwrap()) as f64;
            let length = if max_line > can_heights.length().try_into().unwrap() {
                can_heights.length() - 1
            } else {
                (max_line - 1).try_into().unwrap()
            };

            let last_line_heights:Vec<_> = can_heights.get(length).dyn_into::<Array>().unwrap().into_iter().collect();
            
            let mut last_words = Vec::new();
            let mut last_width = 0.0;

            for width in last_line_heights {
                last_width += width;
                if last_width + ellipsis_width > self.line_clamp_props.content_width as f64 {
                    last_words.push(-1.0);
                    break;
                } else {
                    last_words.push(*width);
                }
            }

            if last_words.last() != Some(&-1.0) {
                last_words.push(-1.0);
            }

            let mut modified_heights = can_heights.clone();
            modified_heights[length] = last_words;
            
            let mut i = 0;
            let mut html = Vec::new();
            
            for item in modified_heights {
                let mut str = String::new();
                for t in item {
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
                font_size: words_info.font_size as i32,
            }
        } else {
            let mut i = 0;
            let mut html = Vec::new();
            
            for item in can_heights {
                let mut str = String::new();
                for _ in item {
                    str = format!("{} {} ", str, self.line_clamp_props.texts[i]);
                    i += 1;
                }
                html.push(self.transaction_to_html(&str));
            }

            InputResultForEllipsis {
                html,
                font_size: words_info.font_size as i32,
            }
        }
    }


    pub fn transaction_to_html(&self, value: &str) -> String {
        let format_html = format!("<span style='display:inline-block;'>{}</span>", value.trim());
        format_html
    }

    pub fn ellipsis_result(&self) -> Result<InputResultForEllipsis, String> {
        let mut html_result = Vec::new();
        let html: String = self.transaction_to_html(&self.ellipsis);
        html_result.push(html);
        Ok(InputResultForEllipsis {
            html: html_result,
            font_size: self.min_font_size
        })
    }

    pub fn get_word_widths(&self, font_size: i32, incompleteWidth: Vec<i32>) -> IWordInfo {
        if font_size < self.min_font_size {
            return IWordInfo {
                text: self.line_clamp_props.texts[0..incompleteWidth.len()].to_vec(),
                ellipsis:true,
                widths: incompleteWidth,
                font_size: self.min_font_size as usize
            }
        }else {
            let mut widths:Vec<f64> = self.line_clamp_props.texts.iter().map( |t| {
                self.get_width_of_content(t.clone(), font_size)
            }).collect();
            let space_width = self.calc_space_width(font_size);
            let width_len = widths.len() - 1;
             widths = widths.into_iter()
                .enumerate()
                .map(|(index, w)| {
                    if width_len != index { w + space_width } else { w }
                })
                .collect();
            if widths.iter().all(|&w| w <= self.line_clamp_props.content_width) {
                return IWordInfo {
                    text: self.line_clamp_props.texts.clone(),
                    widths,
                    font_size: font_size.try_into().unwrap(),
                    ellipsis: false,
                };
            }

            let mut new_incomplete = Vec::new();
            for &w in &widths {
                if w <= self.line_clamp_props.content_width {
                    new_incomplete.push(w);
                } else {
                    break;
                }
            }

            self.get_word_widths(font_size - self.step as i32, new_incomplete)
        }
    }
    
    pub fn get_width_of_content(&self, content: String, fontSize: i32) -> i32{
        let div_element = Rc::new(RefCell::new(self.element.clone()));
        let div_element_add_attr = div_element.clone();
        let div_element_set_inner = div_element.clone();
        let div_element_move = div_element.clone();
        let format_for_style = format!("display:inline-block;font-size:{}px;line-height:{}px", fontSize,self.line_height);
        div_element_add_attr.borrow_mut().set_attribute("style", &format_for_style);
        div_element_set_inner.borrow_mut().set_inner_html(&content);
        let body_tags = document().body().unwrap();
        body_tags.append_child(&div_element_move.borrow());
        let offset_width = <Element as Clone>::clone(&div_element_move.borrow()).dyn_into::<web_sys::HtmlElement>().ok().unwrap().offset_width();
        body_tags.remove_child(&div_element_move.borrow());
        offset_width
    }    
    
    pub fn calc_space_width(&self, fontSize: i32) -> i32{
        let width = self.get_width_of_content(String::from("5 pace"), fontSize);
        let word_width = self.get_width_of_content(String::from("pace"), fontSize);
        let result = width - word_width;
        result
    }

    pub fn calc_word_width_can_in_content(&self, result: &IWordInfo, content_width: f64) 
    -> Array
     {
     let widths = result.widths.clone();
     let mut groups:Vec<Vec<f64>> = Vec::new();
     let mut current_group:Vec<f64> =  Vec::new();
     let mut current_sum:f64 = 0.0;

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

        groups.iter().into_iter().map(|i| {
            let arr = Array::new();
            for &num in i {
                arr.push(&JsValue::from_f64(num));
            }
            arr
        }).collect()
    }
}

