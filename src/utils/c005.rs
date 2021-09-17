/// 题目：输入三个整数x,y,z，请把这三个数由小到大输出。
///
/// 程序分析：我们想办法把最小的数放到x上，先将x与y进行比较，如果x>y则将x与y的值进行交换，然后再用x与z进行比较，如果x>z则将x与z的值进行交换，这样能使x最小。
pub fn test() -> Vec<isize> {
  let mut nums = String::new();
  println!("请输入整数，以空格分隔:");
  std::io::stdin().read_line(&mut nums).unwrap();
  let mut nums = nums
    .replace("\r\n", "")
    .split_whitespace()
    .collect::<Vec<&str>>()
    .iter()
    .map(|x| x.parse::<isize>().unwrap())
    .collect::<Vec<isize>>();
  nums.sort_by(|a, b| a.cmp(&b));

  println!("{:?}", nums);

  nums
}
