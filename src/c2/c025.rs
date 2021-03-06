/// 题目：求1+2!+3!+...+20!的和。
///
/// 程序分析：此程序只是把累加变成了累乘。
pub fn test() -> usize {
  let (mut sum, mut mix) = (0, 1);
  for i in 1..21 {
    mix *= i;
    sum += mix;
  }

  sum
}