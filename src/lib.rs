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
    pub contentWidth: i32,
    pub contentHeight: i32,
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
    pub minFontSize: i32,
    pub maxFontSize: i32,
    pub lineHeight: i32,
    pub minWidthHeight: i32,
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
            minFontSize: 10,
            maxFontSize: 14,
            lineHeight: 14,
            minWidthHeight: 5,
            ellipsis: String::from("..."),
            line_clamp_props
        }
    }

    pub fn calc(&mut self) 
    // -> InputResultForEllipsis
    {
        // if (self.line_clamp_props.contentHeight <= self.minWidthHeight ) 
        // | (self.line_clamp_props.contentWidth <= self.minWidthHeight) {
        //    return self.ellipsisResult()
        // } else {
        //     let wordInfo = self.getWordWidths(self.maxFontSize, Vec::new());
        //     // let canWidths = self.calc_word_width_can_in_content(wordInfo, self.line_clamp_props.c);
        //     // let max_line = Math::floor(self.contentHeight / self.lineHeight);

        //     return self.ellipsisResult()
        // }
    }

    pub fn transactionToHTML(&self, value: &str) -> String {
        let format_html = format!("<span style='display:inline-block;'>{}</span>", value.trim());
        format_html
    }

    pub fn ellipsisResult(&self) -> Result<InputResultForEllipsis, String> {
        let mut html_result = Vec::new();
        let html: String = self.transactionToHTML(&self.ellipsis);
        html_result.push(html);
        Ok(InputResultForEllipsis {
            html: html_result,
            font_size: self.minFontSize
        })
    }

    pub fn getWordWidths(&self, font_size: i32, incompleteWidth: Vec<i32>) -> IWordInfo {
        if font_size < self.minFontSize {
            return IWordInfo {
                text: self.line_clamp_props.texts.splice(0, incompleteWidth.len()),
                ellipsis:true,
                widths: incompleteWidth,
                font_size: self.minFontSize
            }
        }else {
            
            return IWordInfo {
                text: self.line_clamp_props.texts.splice(0, incompleteWidth.len()),
                ellipsis:true,
                widths: incompleteWidth,
                font_size: self.minFontSize
            }
        }
    }
    
    pub fn getWidthOfContent(&self, content: String, fontSize: i32) -> i32{
        let div_element = Rc::new(RefCell::new(self.element.clone()));
        let div_element_add_attr = div_element.clone();
        let div_element_set_inner = div_element.clone();
        let div_element_move = div_element.clone();
        let format_for_style = format!("display:inline-block;font-size:{}px;line-height:{}px", fontSize,self.lineHeight);
        div_element_add_attr.borrow_mut().set_attribute("style", &format_for_style);
        div_element_set_inner.borrow_mut().set_inner_html(&content);
        let body_tags = document().body().unwrap();
        body_tags.append_child(&div_element_move.borrow());
        let offset_width = <Element as Clone>::clone(&div_element_move.borrow()).dyn_into::<web_sys::HtmlElement>().ok().unwrap().offset_width();
        body_tags.remove_child(&div_element_move.borrow());
        offset_width
    }    
    
    pub fn calc_space_width(&self, fontSize: i32) -> i32{
        let width = self.getWidthOfContent(String::from("5 pace"), fontSize);
        let word_width = self.getWidthOfContent(String::from("pace"), fontSize);
        let result = width - word_width;
        result
    }

    pub fn calc_word_width_can_in_content(result: &IWordInfo, content_width: f64) 
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

