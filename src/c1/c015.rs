/// 题目：利用条件运算符的嵌套来完成此题：学习成绩>=90分的同学用A表示，60-89分之间的用B表示，60分以下的用C表示。
///
/// 程序分析：(a>b)?a:b这是条件运算符的基本例子。
pub fn test() -> char {
  let mut score = String::new();
  println!("请输入分数:");
  std::io::stdin().read_line(&mut score).unwrap();
  let score = score.replace("\r\n", "").parse::<i32>().unwrap();

  let res = if score >= 90 {
    'A'
  } else {
    (score >= 60).then(|| 'B').unwrap_or('C')
  };

  println!("学习成绩: {}", res);
  res
}
