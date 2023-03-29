use std::collections::VecDeque;

fn find(tomatoes : &mut Vec<Vec<i32>>, x : i32, y : i32, start : Vec<(i32, i32)>) -> i32{

    let dx = [0, 0, -1, 1];
    let dy = [-1, 1, 0, 0];

    let mut queue = VecDeque::new();
    for (_, v) in start.iter().enumerate(){
        queue.push_back((v.0, v.1, 0));
    }
    
    loop {
        let t = queue.pop_front().unwrap();
        
        for i in 0..4{
            let next = (t.0 + dx[i], t.1 + dy[i]);
            if next.0 >= 0 && next.0 < x && next.1 >= 0 && next.1 < y{
                if tomatoes[next.1 as usize][next.0 as usize] == 0{
                    tomatoes[next.1 as usize][next.0 as usize] = 1;
                    queue.push_back((next.0, next.1, t.2 + 1));
                }
            }
        }
        let mut complete = true;

        if queue.is_empty(){
            for (_, v1) in tomatoes.iter().enumerate(){
                if v1.contains(&0){
                    complete = false;
                }
            }
            if complete{
                return t.2;
            }else{
                return -1;
            }
        }
    }
}

fn main(){
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();

    let t = buf.split_whitespace().map(|v| v.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let (x, y) = (t[0], t[1]);
    buf.clear();

    let mut tomatoes = vec![vec![0;x as usize];y as usize];
    let mut start = Vec::<(i32, i32)>::new();

    for i in 0..y{
        stdin.read_line(&mut buf).unwrap();
        let t = buf.split_whitespace().map(|v| v.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        for (j, v) in t.iter().enumerate(){
            tomatoes[i as usize][j as usize] = *v;
            if *v == 1{
                start.push((j as i32, i));
            }
        }
        buf.clear();
    }
    if start.len() == 0{
        print!("-1");
    }else{
        println!("{:?}", find(&mut tomatoes, x, y, start));
    }
    
}
