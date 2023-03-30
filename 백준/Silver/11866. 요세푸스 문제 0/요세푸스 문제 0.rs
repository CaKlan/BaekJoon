use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();
    
    let t : Vec<i32> = buf.split_whitespace().map(|v| v.parse::<i32>().unwrap()).collect();
    let (n, k) = (t[0], t[1] as usize);

    let mut humans : VecDeque<i32> = (1..=n).collect();
    let mut stack : VecDeque<i32> = VecDeque::new();
    let mut order = k - 1;

    for _ in 0..n{
        if !humans.is_empty(){
            let rm = humans.remove(order).unwrap();
            stack.push_back(rm);
            if humans.is_empty(){
                break;
            }
            if order + k - 1 >= humans.len(){
                order = (order + k - 1 - humans.len()) % humans.len();
            }else{
                order += k - 1;
            }
        }
    }

    print!("<");
    for i in 0..n-1{
        print!("{}, ", stack[i as usize]);
    }
    print!("{}", stack[stack.len()-1]);
    print!(">");
}
