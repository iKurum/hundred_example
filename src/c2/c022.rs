/// 题目：两个乒乓球队进行比赛，各出三人。甲队为a,b,c三人，乙队为x,y,z三人。已抽签决定比赛名单。有人向队员打听比赛的名单。a说他不和x比，c说他不和x,z比，请编程序找出三队赛手的名单。
pub fn test() -> Vec<(String, String)> {
  let mut team = vec![("", "x"), ("", "y"), ("", "z")];

  for i in 0..team.len() {
    if team[i].1 != "x" {
      if team[i].1 != "z" {
        team[i].0 = "c";
      } else {
        team[i].0 = "a";
      }
    } else {
      team[i].0 = "b";
    }
  }

  team.sort_by(|a, b| a.0.cmp(b.0));
  team
    .iter()
    .map(|x| (x.0.to_string(), x.1.to_string()))
    .collect::<Vec<(String, String)>>()
}
