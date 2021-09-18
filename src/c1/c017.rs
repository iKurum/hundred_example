/// 题目：输入一行字符，分别统计出其中英文字母、空格、数字和其它字符的个数。
///
/// 程序分析：利用while语句,条件为输入的字符不为'\n'。(windows为'\r\n')
pub fn test() -> Result<Vec<i32>, ()> {
  let mut s = String::new();
  println!("请输入一些字符:");
  std::io::stdin().read_line(&mut s).expect("读取输入错误");
  let s = s
    .replace("\r\n", "")
    .chars()
    .map(|x| x as u32)
    .collect::<Vec<u32>>();

  let (mut letters, mut spaces, mut digits, mut others) = (0, 0, 0, 0);
  for i in s {
    if i == 32 {
      spaces += 1;
    } else if i >= 48 && i <= 57 {
      digits += 1;
    } else if (i >= 65 && i <= 90) || (i >= 97 && i <= 122) {
      letters += 1;
    } else {
      others += 1;
    }
  }
  println!(
    "字母={},数字={},空格={},其他={}",
    letters, digits, spaces, others
  );

  Ok(vec![letters, digits, spaces, others])
}
