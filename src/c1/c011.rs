/// 题目：古典问题（兔子生崽）：有一对兔子，从出生后第3个月起每个月都生一对兔子，小兔子长到第三个月后每个月又生一对兔子，假如兔子都不死，问每个月的兔子总数为多少？（输出前40个月即可）
///
/// 程序分析：兔子的规律为数列1,1,2,3,5,8,13,21....，即下个月是上两个月之和（从第三个月开始）。
pub fn test() -> usize {
  let mut month = String::new();
  println!("请输入检查多少个月:");
  std::io::stdin()
    .read_line(&mut month)
    .expect("读取输入错误");
  let month = month
    .replace("\r\n", "")
    .parse::<i32>()
    .expect("输入非数字");
  let (mut size, mut f1, mut f2) = (2, 1, 1);

  println!("前两个月不生产，总数为 2");
  if month < 2 {
    return 2;
  } else {
    for i in 3..(month + 1) {
      println!("第{}个月新生产数: {}", i, f2);
      size += f2;

      if i > 3 {
        let t = f1;
        f1 = f2;
        f2 += t;
      }
    }
    println!("总计{}个月，总数为: {}", month, size);
    return size;
  }
}
