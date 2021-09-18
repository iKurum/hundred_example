/// 题目：利用递归方法求5!。
///
/// 程序分析：递归公式：fn=fn_1*4!
pub fn test() {
  for i in 0..6 {
    println!("{}! = {}", i, fact(i));
  }
}

fn fact(n: usize) -> usize {
  if n == 0 {
    1
  } else {
    n * fact(n - 1)
  }
}
