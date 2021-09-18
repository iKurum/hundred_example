//
//    *
//   ***
//  *****
// *******
//  *****
//   ***
//    *

/// 题目：打印出图案（菱形）。
pub fn test() {
  let a = vec![3, 2, 1, 0, 1, 2, 3];

  for i in a {
    if i != 0 {
      println!("{}{}", " ".repeat(i), "*".repeat(7 - i * 2));
    } else {
      println!("{}", "*".repeat(7));
    }
  }
}
