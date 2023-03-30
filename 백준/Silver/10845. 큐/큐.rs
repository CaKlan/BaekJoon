use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();

    let n = buf.trim().parse::<i32>().unwrap();
    let mut arr : VecDeque<i32> = VecDeque::new();
    buf.clear();

    for _ in 0..n{
        stdin.read_line(&mut buf).unwrap();
        let t : Vec<&str> = buf.split_whitespace().collect();

        match t[0]{
            "push" => arr.push_back(t[1].parse::<i32>().unwrap()),
            "pop" => match arr.pop_front(){
                Some(t) => println!("{}", t),
                None => println!("-1"),
            },
            "size" => println!("{}", arr.len()),
            "empty" => println!("{}", if arr.is_empty() {1} else {0}),
            "front" => if arr.is_empty() {println!("-1")} else {println!("{}", arr.front().unwrap())},
            "back" => if arr.is_empty() {println!("-1")} else {println!("{}", arr.back().unwrap())},
            _ => (),
        }
        buf.clear();
    }
}
