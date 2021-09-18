/// 题目：判断101到200之间的素数。
///
/// 程序分析：判断素数的方法：用一个数分别去除2到sqrt(这个数)，如果能被整除，则表明此数不是素数，反之是素数。
pub fn test() -> Vec<i32> {
  let mut res: Vec<i32> = Vec::new();
  for i in 101..201 {
    let mut has = true;
    for j in 2..i {
      if i % j == 0 {
        has = false;
        break;
      }
    }
    if has {
      res.push(i);
    }
  }

  println!("101到200之间的素数: {:?}", res);
  res
}