use std::collections::VecDeque;

fn main(){
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();

    let t = buf.split_whitespace().map(|v| v.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    buf.clear();
    let (x, y) = (t[1], t[0]);

    let mut map = vec![vec![0; x as usize];y as usize];

    for i in 0..y as usize{
        stdin.read_line(&mut buf).unwrap();
        for (j, v) in buf.trim().bytes().into_iter().enumerate(){
            map[i][j] = (v - b'0') as i32;
        }

        buf.clear();
    }

    print!("{}", find(&mut map, x, y));
}

fn find(map : &mut Vec<Vec<i32>>, x : i32, y : i32) -> i32{

    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; x as usize]; y as usize];
    let mut visited_broken = vec![vec![false; x as usize]; y as usize];
    let dx = [0,  0, 1, -1];
    let dy = [1, -1, 0,  0];

    queue.push_back((0, 0, 0, 0));
    visited[0][0] = true;

    loop {
        let t = queue.pop_front().unwrap(); // (x, y, cnt, isBroken)
        if (t.0, t.1) == (x-1, y-1){
            return t.2 + 1;
        }
        for i in 0..4{
            let next = (t.0 + dx[i], t.1 + dy[i]);

            if next == (x-1, y-1){
                return t.2 + 2;
            }

            if 0 <= next.0 && next.0 < x && 0 <= next.1 && next.1 < y {
                if t.3 == 0{
                    if visited[next.1 as usize][next.0 as usize] == false {
                        if map[next.1 as usize][next.0 as usize] == 0{
                            visited[next.1 as usize][next.0 as usize] = true;
                            queue.push_back((next.0, next.1, t.2 + 1, 0));
                        }if map[next.1 as usize][next.0 as usize] == 1{
                            visited_broken[next.1 as usize][next.0 as usize] = true;
                            queue.push_back((next.0, next.1, t.2 + 1, 1));
                        }
                    }
                }else if t.3 == 1{
                    if visited_broken[next.1 as usize][next.0 as usize] == false {
                        if map[next.1 as usize][next.0 as usize] == 0{
                            visited_broken[next.1 as usize][next.0 as usize] = true;
                            queue.push_back((next.0, next.1, t.2 + 1, 1));
                        }
                    }
                }
            }
        }
        
        if queue.is_empty(){
            return -1;
        }
    }
}