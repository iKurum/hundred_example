/// 题目：请输入星期几的第一个字母来判断一下是星期几，如果第一个字母一样，则继续判断第二个字母。
///
/// 程序分析：用情况语句比较好，如果第一个字母一样，则判断用情况语句或if语句判断第二个字母。
pub fn test() {
  use std::io::stdin;

  let mut s = String::new();
  println!("请输入第一个字母:");
  stdin().read_line(&mut s).expect("输入读取错误");
  let res = s.replace("\r\n", "");
  match &res.to_lowercase()[..] {
    "m" => println!("monday"),
    "w" => println!("wednesday"),
    "f" => println!("friday"),
    "t" => {
      println!("请输入下一个字母:");
      stdin().read_line(&mut s).expect("输入读取错误");
      let res = s.replace("\r\n", "");
      match &res.to_lowercase()[..] {
        "tu" => println!("tuesday"),
        "th" => println!("thursday"),
        _ => println!("error"),
      }
    }
    "s" => {
      println!("请输入下一个字母:");
      stdin().read_line(&mut s).expect("输入读取错误");
      let res = s.replace("\r\n", "");
      println!("{}", res);
      match &res.to_lowercase()[..] {
        "sa" => println!("saturday"),
        "su" => println!("sunday"),
        _ => println!("error"),
      }
    }
    _ => println!("error"),
  }
}
