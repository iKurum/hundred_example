/// 题目：输出9*9口诀。
///
/// 程序分析：分行与列考虑，共 9 行 9 列，i 控制行，j 控制列。
pub fn test() {
  for i in 1..10 {
    for j in 1..(i + 1) {
      print!("{}*{}={}  ", j, i, i * j);
    }
    println!();
  }
}
