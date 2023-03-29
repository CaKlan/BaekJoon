use std::collections::VecDeque;

fn main(){
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();

    let t = buf.split_whitespace().map(|v| v.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    buf.clear();

    let (ladder, snake) = (t[0], t[1]);
    
    let mut teleport = Vec::<(i32, i32)>::new();

    for i in 0..(ladder + snake){
        stdin.read_line(&mut buf).unwrap();
        let t = buf.split_whitespace().map(|v| v.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        teleport.push((t[0], t[1]));
        buf.clear();
    }
    print!("{}", find(&teleport));
}

fn find(teleport : &Vec<(i32, i32)>) -> i32{

    let mut queue = VecDeque::new();
    let mut visited = vec![false; 101];

    queue.push_back((1, 0));

    loop {
        let t = queue.pop_front().unwrap();

        for dice in 1..=6{
            let mut next = (t.0 + dice, t.1 + 1);

            for (_, v) in teleport.iter().enumerate(){
                if v.0 == next.0{
                    next.0 = v.1;
                }
            }

            if next.0 == 100{
                return next.1;
            }

            if visited[next.0 as usize] == false{
                visited[next.0 as usize] = true;
                queue.push_back(next);
            }
        }
    }
}