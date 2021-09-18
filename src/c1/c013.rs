/// 题目：打印出所有的"水仙花数"，所谓"水仙花数"是指一个三位数，其各位数字立方和等于该数本身。
/// 
/// 例如：153是一个"水仙花数"，因为153=1的三次方＋5的三次方＋3的三次方。
///
/// 程序分析：利用for循环控制100-999个数，每个数分解出个位，十位，百位。
pub fn test() -> Vec<i32> {
  let mut res: Vec<i32> = Vec::new();
  for i in 100..1000 {
    let x = i % 10;
    let y = i / 10 % 10;
    let z = i / 100 % 10;

    if i == (x * x * x + y * y * y + z * z * z) {
      res.push(i);
    }
  }

  println!("100-999的水仙花数: {:?}", res);
  res
}