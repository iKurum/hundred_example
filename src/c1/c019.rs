/// 题目：一个数如果恰好等于它的因子之和，这个数就称为"完数"。例如6=1＋2＋3.编程找出1000以内的所有完数。
///
/// 程序分析：请参照：C 练习实例14。
pub fn test() -> Result<Vec<i32>, ()> {
  let mut res: Vec<i32> = Vec::new();
  for i in 2..1001 {
    let mut sum = 1;
    let mut t: Vec<i32> = vec![1];

    for j in 2..(i / 2 + 1) {
      if i % j == 0 {
        sum += j;
        t.push(j);
      }
    }

    if i == sum {
      res.push(i);

      let mut r = String::new();
      for i in 0..t.len() {
        if i == 0 {
          r += &t[i].to_string()
        } else {
          r += &format!("+{}", t[i]);
        }
      }
      println!("{}={}", i, r);
    }
  }

  Ok(res)
}
