/// 题目：有1、2、3、4个数字，能组成多少个互不相同且无重复数字的三位数？都是多少？
///
/// 程序分析：可填在百位、十位、个位的数字都是1、2、3、4。组成所有的排列后再去 掉不满足条件的排列。
pub fn test() -> Vec<i32> {
  let mut res: Vec<i32> = Vec::new();

  for i in 1..5 {
    for j in 1..5 {
      for k in 1..5 {
        if i != j && i != k && j != k {
          res.push(i * 100 + j * 10 + k);
        }
      }
    }
  }

  println!("{:?}", res);

  res
}
