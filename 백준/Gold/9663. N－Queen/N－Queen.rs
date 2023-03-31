use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();
    
    let n = buf.trim().parse::<i32>().unwrap();
    
    let mut arr = vec![-1; n as usize];
    let mut count = 0;
    solution(0, n, &mut arr, &mut count);

    print!("{}", count);
}

fn solution(col : i32, n : i32, arr : &mut Vec<i32>, count : &mut i32){

    if col == n{
        *count += 1;
        return;
    }

    for i in 0..n{
        arr[col as usize] = i;
        if invalid(col, &arr){
            solution(col + 1, n, arr, count);
        }
    }
}

fn invalid(col : i32, arr : &Vec<i32>) -> bool{
    for i in 0..col{
        if arr[col as usize] == arr[i as usize] || (arr[col as usize] - arr[i as usize]).abs() == (col - i){
            return false;
        }
    }

    true
}