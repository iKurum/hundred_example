/// 题目：利用递归函数调用方式，将所输入的5个字符，以相反顺序打印出来。
pub fn test() -> String {
  let mut s = String::new();
  println!("请输入字符:");
  std::io::stdin().read_line(&mut s).expect("输入读取错误");
  s.replace("\r\n", "").chars().rev().collect()
}