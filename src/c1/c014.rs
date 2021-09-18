/// 题目：将一个正整数分解质因数。例如：输入90,打印出90=2*3*3*5。
///
/// 程序分析：对n进行分解质因数，应先找到一个最小的质数k，然后按下述步骤完成：
/// 1. 如果这个质数恰等于（小于的时候，继续执行循环）n，则说明分解质因数的过程已经结束，另外 打印出即可。
/// 2. 但n能被k整除，则应打印出k的值，并用n除以k的商,作为新的正整数n.重复执行第二步。
/// 3. 如果n不能被k整除，则用k+1作为k的值,重复执行第一步。
pub fn test() -> Vec<i32> {
  let mut res: Vec<i32> = Vec::new();
  let mut num = String::new();
  println!("请输入整数:");
  std::io::stdin().read_line(&mut num).expect("读取输入错误");
  let mut num = num
    .replace("\r\n", "")
    .parse::<isize>()
    .expect("输入非数字");
  let n = num;
  for i in 2..(num + 1) {
    while num % i == 0 {
      res.push(i as i32);
      num /= i;
    }
  }

  if res.len() != 0 {
    let mut r = String::new();
    for i in 0..res.len() {
      if i == 0 {
        r += &res[i].to_string()
      } else {
        r += &format!("*{}", res[i]);
      }
    }

    println!("{}={}", n, r);
  }
  res
}
