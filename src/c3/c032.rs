/// 题目：删除一个字符串中的指定字母，如：字符串 "aca"，删除其中的 a 字母。
pub fn test() {
  use std::io::stdin;

  let mut s = String::new();
  println!("请输入原始字符串:");
  stdin().read_line(&mut s).expect("输入读取错误");

  let mut c = String::new();
  println!("请输入要删除的字符:");
  stdin().read_line(&mut c).expect("输入读取错误");
  let c = c.replace("\r\n", "");

  println!("删除{}字符: {}", c, s.replace(&c, ""));
}