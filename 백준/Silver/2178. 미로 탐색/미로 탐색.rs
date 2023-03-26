use std::collections::{vec_deque::VecDeque, HashMap};

fn check(maze : &Vec<Vec<bool>>, idx : (i32, i32)) -> bool {
    match maze.get(idx.0 as usize){
        Some(m1) => {
            match m1.get(idx.1 as usize) {
                Some(m2) => {
                    *m2
                },
                None => {
                    false
                },
            }
        },
        None => {
            false
        },
    }
}

fn connect(hashmap : &mut HashMap<(i32, i32), Vec<(i32, i32)>>, maze : &mut Vec<Vec<bool>>){
    let (n, m) = (maze.len(), maze[0].len());
    for i in 0..n{
        for j in 0..m{
            if maze[i][j] {
                if check(&maze, (i as i32, j as i32 - 1)){
                    hashmap.get_mut(&(i as i32, j as i32)).unwrap().push((i as i32, j as i32 - 1));
                }
                if check(&maze, (i as i32, j as i32 + 1)){
                    hashmap.get_mut(&(i as i32, j as i32)).unwrap().push((i as i32, j as i32 + 1));
                }
                if check(&maze, (i as i32 - 1, j as i32)){
                    hashmap.get_mut(&(i as i32, j as i32)).unwrap().push((i as i32 - 1, j as i32));
                }
                if check(&maze, (i as i32 + 1, j as i32)){
                    hashmap.get_mut(&(i as i32, j as i32)).unwrap().push((i as i32 + 1, j as i32));
                }
            }
        }
    }
}

fn find(hashmap : &mut HashMap<(i32, i32), Vec<(i32, i32)>>, n : i32, m : i32) -> i32{
    let mut stack = VecDeque::<(i32, i32, i32)>::new();
    let mut result = 0;

    let mut visited : Vec<Vec<bool>> = vec![vec![false; 100]; 100];

    let a = hashmap.get(&(0, 0)).unwrap();
    visited[0][0] = true;

    for (_, v) in a.iter().enumerate(){
        stack.push_back((v.0, v.1, 2));
    }

    while !stack.is_empty() {
        let next = stack.pop_front().unwrap();
        
        visited[next.0 as usize][next.1 as usize] = true;

        let a = hashmap.get(&(next.0, next.1)).unwrap();

        for (_, v) in a.iter().enumerate(){
            if !stack.contains(&(v.0, v.1, next.2 + 1)) && !visited[v.0 as usize][v.1 as usize]{
                stack.push_back((v.0, v.1, next.2 + 1));
            }
        }
        // println!("{:?}", next);
        if next.0 == (n-1) && next.1 == (m-1){
            result = next.2;
            return result;
        }
    }

    result
}

fn main(){
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();

    let t = buf.split_whitespace().map(|v| v.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let (n, m) = (t[0], t[1]);
    buf.clear();

    let mut maze = vec![vec![false; m as usize]; n as usize];
    for i in 0..n{
        stdin.read_line(&mut buf).unwrap();
        let line : Vec<bool> = buf.bytes().map(|v| if v == b'0' { false } else { true }).collect();
        for j in 0..m{
            maze[i as usize][j as usize] = line[j as usize];
        }
        buf.clear();
    }

    let mut hashmap = HashMap::new();

    for i in 0..n{
        for j in 0..m{
            hashmap.insert((i, j), Vec::<(i32, i32)>::new());
        }
    }

    connect(&mut hashmap, &mut maze);
    println!("{:?}", find(&mut hashmap, n, m));

}
