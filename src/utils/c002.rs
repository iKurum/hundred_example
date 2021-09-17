/// 题目：企业发放的奖金根据利润提成。
///
/// 利润(I)低于或等于10万元时，奖金可提10%；
///
/// 利润高于10万元，低于20万元时，低于10万元的部分按10%提成，高于10万元的部分，可提成7.5%；
///
/// 20万到40万之间时，高于20万元的部分，可提成5%；
///
/// 40万到60万之间时高于40万元的部分，可提成3%；
///
/// 60万到100万之间时，高于60万元的部分，可提成1.5%；
///
/// 高于100万元时，超过100万元的部分按1%提成。
///
/// 从键盘输入当月利润I，求应发放奖金总数？
///
/// 程序分析：请利用数轴来分界，定位。注意定义时需把奖金定义成双精度浮点(double)型。
pub fn test() -> f32 {
  let mut profit = String::new();
  println!("请输入当月利润: ");
  std::io::stdin().read_line(&mut profit).unwrap();

  let profit = profit.replace("\r\n", "").parse::<f32>().unwrap();

  set_bonus(profit)
}

fn set_bonus(profit: f32) -> f32 {
  let bonus_list = vec![
    100_000.0 * 0.1,
    100_000.0 * 0.1 + 100_000.0 * 0.075,
    100_000.0 * 0.1 + 100_000.0 * 0.075 + 200_000.0 * 0.05,
    100_000.0 * 0.1 + 100_000.0 * 0.075 + 200_000.0 * 0.05 + 200_000.0 * 0.03,
    100_000.0 * 0.1 + 100_000.0 * 0.075 + 200_000.0 * 0.05 + 200_000.0 * 0.03 + 400_000.0 * 0.015,
  ];
  println!("输入的利润为: {}", profit);

  let mut bonus: f32 = 0.0;
  if profit <= 100_000.0 {
    bonus = profit * 0.1;
  } else if profit <= 200_000.0 {
    bonus = bonus_list[0] + (profit - 100_000.0) * 0.075;
  } else if profit <= 400_000.0 {
    bonus = bonus_list[1] + (profit - 200_000.0) * 0.05;
  } else if profit <= 600_000.0 {
    bonus = bonus_list[2] + (profit - 400_000.0) * 0.03;
  } else if profit <= 1_000_000.0 {
    bonus = bonus_list[3] + (profit - 600_000.0) * 0.015;
  } else if profit > 1_000_000.0 {
    bonus = bonus_list[4] + (profit - 1_000_000.0) * 0.1;
  }

  println!("奖金为: {:?}", bonus);

  bonus
}
