fn main(){
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap(); // 입력 받는 함수

    let v = buf.trim().split_whitespace().map(|v| v.parse::<i32>().unwrap()).collect::<Vec<i32>>(); // 공백을 구분으로 벡터에 저장

    let (N, K) = (v[0], v[1]); // N : 동전 종류, K : 가치의 합

    buf.clear(); //버퍼 재사용을 위해 클리어

    let mut coin_list : Vec<i32> = Vec::new(); //코인 종류 리스트 ex:[1, 10, 50, 100, ...]

    let mut remain = K; // 동전으로 빼고 남은 수
    let mut count = 0;  // 사용한 동전 개수

    for _ in 0..N{ // N회 실행
        stdin.read_line(&mut buf).unwrap(); // 입력 받는 함수
        coin_list.push(buf.trim().parse::<i32>().unwrap()); // 코인 입력받아 리스트에 넣어줌
        buf.clear(); //버퍼 재사용을 위해 클리어
    }

    while remain != 0 { // 남은 수가 0이 될때 까지
        for i in (0..N).rev(){ //큰수부터 함
            while coin_list[i as usize] <= remain{
                remain -= coin_list[i as usize];
                count += 1;
            }
        }
    }

    print!("{:?}", count);
}