/// 题目：一个5位数，判断它是不是回文数。即12321是回文数，个位与万位相同，十位与千位相同。
///
/// 程序分析：学会分解出每一位数。
pub fn test() -> bool {
  let mut num = String::new();
  println!("请输入一个5位的数字:");
  std::io::stdin().read_line(&mut num).expect("输入读取错误");
  let num = num
    .replace("\r\n", "")
    .chars()
    .map(|x| x.to_digit(10).expect("输入数字错误"))
    .collect::<Vec<u32>>();

  let l = num.len();
  for i in 0..(l / 2) {
    if num[i] != num[l - 1 - i] {
      println!("不是回文数");
      return false;
    }
  }
  println!("是回文数");
  true
}
