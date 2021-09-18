/// 题目：求s=a+aa+aaa+aaaa+aa...a的值，其中a是一个数字。例如2+22+222+2222+22222(此时共有5个数相加)，几个数相加有键盘控制。
///
/// 程序分析：关键是计算出每一项的值。
pub fn test() -> Result<usize, String> {
  let mut s = String::new();
  println!("请输入a 和 n (空格分隔):");
  std::io::stdin().read_line(&mut s).expect("读取输入错误");

  let mut s = s
    .replace("\r\n", "")
    .split_whitespace()
    .map(|x| x.parse::<usize>().expect("输入的数字有误"))
    .collect::<Vec<usize>>();

  let mut res: usize = 0;
  let i = s[0];
  if s.len() == 2 {
    while s[1] > 0 {
      res += s[0];
      s[0] = i + s[0] * 10;
      s[1] -= 1;
    }

    println!("a+aa+...={}", res);
    Ok(res)
  } else {
    Err("输入的 a 或 n 有误".to_string())
  }
}
