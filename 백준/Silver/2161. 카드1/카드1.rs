use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();
    
    let n = buf.trim().parse::<i32>().unwrap();
    
    let mut cards : VecDeque<i32> = (1..=n).collect();
    let mut stack : VecDeque<i32> = VecDeque::new();

    while !cards.is_empty(){
        let pop = cards.pop_front().unwrap();
        stack.push_back(pop);
        if !cards.is_empty(){
            let pop = cards.pop_front().unwrap();
            cards.push_back(pop);
        }
    }

    for i in 0..n{
        print!("{} ", stack[i as usize]);
    }
}
