/// 题目：输入某年某月某日，判断这一天是这一年的第几天？
///
/// 程序分析：以3月5日为例，应该先把前两个月的加起来，然后再加上5天即本年的第几天，特殊情况，闰年且输入月份大于3时需考虑多加一天。
pub fn test() {
  let mut date = String::new();
  println!("请输入年、月、日，格式为：年-月-日（2015-12-10）:");
  std::io::stdin().read_line(&mut date).unwrap();
  date = date.replace("\r\n", "");
  let d = date.split("-").collect::<Vec<&str>>();

  if d.len() == 3 {
    let year = d[0].parse::<u32>().unwrap();
    let month = d[1].parse::<u32>().unwrap();
    let day = d[2].parse::<u32>().unwrap();

    if month > 12 {
      println!("month 输入错误 ...");
    } else {
      let (mdays, days) = month_days(is_leap(year), month);

      if day > mdays[(month - 1) as usize] {
        println!("day 输入错误 ...");
      } else {
        println!("这是这一年的第 {} 天。", days + day);
      }
    }
  }
}

fn is_leap(year: u32) -> bool {
  year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)
}

fn month_days(leap: bool, month: u32) -> (Vec<u32>, u32) {
  let month_d = vec![31, 28, 31, 30, 31, 30, 31, 30, 31, 30, 31, 30, 31];
  let mut d = 0;
  for i in 1..month {
    d += month_d[(i - 1) as usize];
  }

  if leap && month > 2 {
    (month_d, d + 1)
  } else {
    (month_d, d)
  }
}
