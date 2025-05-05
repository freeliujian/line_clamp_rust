use line_clamp::{LineClamp, LineClampProps};

fn main() {
  let lineClampProps = LineClampProps {
    texts: vec![
      String::from("我有一个梦想"),
      String::from("我有一个梦想"),
      String::from("我有一个梦想"),
      String::from("我有一个梦想"),
      String::from("我有一个梦想")
    ],
    contentHeight: 100,
    contentWidth: 200,
  };
  let mut line = LineClamp::new();
  line.init(lineClampProps);
}