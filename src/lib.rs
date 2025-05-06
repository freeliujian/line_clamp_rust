use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::Element;
use gloo::utils::{document};

#[wasm_bindgen]
#[derive(Clone)]
pub struct IWordInfo {
    #[wasm_bindgen(getter_with_clone)] 
    pub text: Vec<String>,
    #[wasm_bindgen(getter_with_clone)] 
    pub widths: Vec<usize>,
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

}

