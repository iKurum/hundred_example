/// 题目：输入两个正整数m和n，求其最大公约数和最小公倍数。
///
/// 程序分析：
///
/// 1. 最小公倍数=输入的两个数之积除于它们的最大公约数，关键是求出最大公约数；
/// 2. 求最大公约数用辗转相除法（又名欧几里德算法）
///     1. 证明：设c是a和b的最大公约数，记为c=gcd(a,b),a>=b,
///
///        令r=a mod b
///
///        设a=kc，b=jc，则k，j互素，否则c不是最大公约数
///
///        据上，r=a-mb=kc-mjc=(k-mj)c
///
///        可知r也是c的倍数，且k-mj与j互素，否则与前述k，j互素矛盾,
///
///        由此可知，b与r的最大公约数也是c，即gcd(a,b)=gcd(b,a mod b)，得证。
///
///     2. 算法描述：
///
///        第一步：a ÷ b，令r为所得余数（0≤r
///
///        第二步：互换：置 a←b，b←r，并返回第一步。
pub fn test() -> Result<(isize, isize), ()> {
  let mut nums = String::new();
  println!("请输入两个数，以空格分隔");
  std::io::stdin().read_line(&mut nums).unwrap();
  let mut nums = nums
    .replace("\r\n", "")
    .split_whitespace()
    .collect::<Vec<&str>>()
    .iter()
    .map(|x| x.parse::<isize>().unwrap())
    .collect::<Vec<isize>>();
  nums.sort_by(|a, b| b.cmp(a));
  
  if nums.len() == 2 {
    let mut r = nums[0] % nums[1];
    let n = nums[0] * nums[1];
    while r != 0 {
      nums[0] = nums[1];
      nums[1] = r;
      r = nums[0] % nums[1];
    }
    println!("这两个数的最大公约数是{}，最小公倍数是{}", nums[1], n / nums[1]);
    Ok((nums[1], n / nums[1]))
  } else {
    println!("请输入两个数，以空格分隔");
    Err(())
  }
}
