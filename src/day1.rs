use aoc_runner_derive::aoc;

#[aoc(day1,part1)]
pub fn solve_part1(lines: &str) -> i32 {

  let mut calories = vec![];
  let mut max_calory = 0;
  let mut current_cal = 0;
  for line in lines.lines() {
    match line.parse::<i32>() {
      Err(_) => {
        calories.push(current_cal);
        if current_cal > max_calory {
          max_calory = current_cal.clone();
        }
        current_cal = 0;
      },
      Ok(v) => current_cal += v,
    };
  }
  max_calory
}
