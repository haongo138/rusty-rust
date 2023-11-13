// ref: https://www.hackerrank.com/challenges/dynamic-array/problem

fn xor(first: i32, second: i32) -> i32 {
  let b_first = format!("{:b}", first);
  let b_second = format!("{:b}", second);

  let max_len = b_first.len().max(b_second.len());
  let b_first_padded = format!("{:0width$b}", first, width=max_len);
  let b_second_padded = format!("{:0width$b}", second, width=max_len);

  let xor_result: String = b_first_padded
    .chars()
    .zip(b_second_padded.chars())
    .map(|(a, b)| if a != b { "1" } else { "0" })
    .collect();
  
  let xor_decimal: i32 = i32::from_str_radix(&xor_result, 2).unwrap();

  xor_decimal
}

fn dynamic_array(n: i32, queries: &[Vec<i32>]) -> Vec<i32> {
  let mut last_answer = 0;
  let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n as usize);
  let mut answers: Vec<i32> = vec![];

  for _ in 0..n {
    arr.push(Vec::new());
  }

  for q in queries {
    let mut idx: i32;
    if q[0] == 1 {
      idx = xor(q[1], last_answer);
      idx = idx % n;
      arr[idx as usize].push(q[2]);
    } else if q[0] == 2 {
      let y_idx: i32;
      idx = xor(q[1], last_answer);
      idx = idx % n;
      y_idx = q[2] % (arr[idx as usize].len() as i32);
      last_answer = arr[idx as usize][y_idx as usize];
      answers.push(last_answer);
    } else {
      println!("Invalid query type");
    }
  }

  answers
}

fn main() {
  let n: i32 = 5;
  let queries = vec![
    vec![1, 1, 3],
    vec![2, 1, 3],
  ];
  let result = dynamic_array(n, &queries);
  println!("Result is {:?}", result);
}