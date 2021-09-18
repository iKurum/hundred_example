/// 题目：给一个不多于5位的正整数，要求：一、求它是几位数，二、逆序打印出各位数字。
///
/// 程序分析：学会分解出每一位数
pub fn test() -> (usize, Vec<u32>) {
  let mut num = String::new();
  println!("请输入不多于5位的正整数:");
  std::io::stdin().read_line(&mut num).expect("输入读取错误");
  let num = num
    .replace("\r\n", "")
    .chars()
    .rev()
    .map(|x| x.to_digit(10).unwrap())
    .collect::<Vec<u32>>();
  (num.len(), num)
}
