use std::rc::Rc;

use wasm_bindgen::prelude::*;
use web_sys::Element;
use gloo::utils::{document};

pub struct IWordInfo {
    pub text: Vec<String>,
    pub widths: Vec<usize>,
    pub font_size: usize,
    pub ellipsis: bool,
}

pub struct LineClampProps {
    pub texts: Vec<String>,
    pub contentWidth: i32,
    pub contentHeight: i32,
}

pub struct InputResultForEllipsis {
    pub html: Vec<String>,
    pub font_size: i32,
}

pub struct LineClamp {
    pub element: Element,
    pub step: f32,
    pub minFontSize: i32,
    pub maxFontSize: i32,
    pub lineHeight: i32,
    pub minWidthHeight: i32,
    pub ellipsis: String,
    pub lineClampProps: LineClampProps,
}

impl LineClamp {
    pub fn new() -> LineClamp {
        LineClamp {
            element: document().create_element("div").unwrap(),
            step: 0.2,
            minFontSize: 10,
            maxFontSize: 14,
            lineHeight: 14,
            minWidthHeight: 5,
            ellipsis: String::from("..."),
            lineClampProps: LineClampProps {
                texts: Vec::new(),
                contentWidth: 1,
                contentHeight: 1,
            },
        }
        
    }

    pub fn init(&mut self, props: LineClampProps) {
        self.lineClampProps = props;
    }

    pub fn calc(&mut self) {

    }

    pub fn transactionToHTML(&self, value: &str) -> String {
        let format_html = format!("<span style='display:inline-block;'>{}</span>", value.trim());
        format_html
    }

    pub fn ellipsisResult(&self) -> InputResultForEllipsis {
        let mut html_result = Vec::new();
        let html: String = self.transactionToHTML(&self.ellipsis);
        html_result.push(html);
        InputResultForEllipsis {
            html: html_result,
            font_size: self.minFontSize
        }
    }

    pub fn getWordWidth(&self, font_size: i32, incompleteWidth: Vec<i32>) {
        // if font_size < self.minFontSize {
        //     IWordInfo {
        //         text: self.lineClampProps.texts,
        //         ellipsis: true,
        //         widths: incompleteWidth,
        //         font_size: self.minFontSize
        //     }
        // }
    }

    pub fn getWidthOfContent(&self, content: String, fontSize: i32) -> i32 {
        let div_element = Rc::new(self.element.clone());
        let div_element_clonet = div_element.clone();
        let format_for_style = format!("display:inline-block;font-size:{}px;line-height:{}px", fontSize,self.lineHeight);
        div_element_clonet.set_attribute("style", &format_for_style);
        div_element_clonet.set_inner_html(&content);
        let body_tags = document().body().unwrap();
        body_tags.append_child(&div_element_clonet);
        let offset_width = <Element as Clone>::clone(&div_element_clonet).dyn_into::<web_sys::HtmlElement>().ok().unwrap().offset_width()
        body_tags.remove_child(&div_element_clonet);
        offset_width
    }    

    pub fn calcSpaceWidth(&self, fontSize: i32) -> i32{
        let width = self.getWidthOfContent(String::from("5 pace"), fontSize);
        let word_width = self.getWidthOfContent(String::from("pace"), fontSize);
        width - word_width
    }
    
    pub fn calc_word_width_can_in_content(result: IWordInfo, contentWidth: i32) {
        // let groups:Vec<i32> = Vec::new();
        // let current_group:Vec<usize> = Vec::new();
        // let current_sum = 0;
        // let widths = result.widths;
        // for i in &widths {
        //     let i = Rc::new(i);
        //     let i_clone = i.clone();
        //    if current_sum + **i_clone <= contentWidth.try_into().unwrap() {
            
        //    }
        // }
    }
}

