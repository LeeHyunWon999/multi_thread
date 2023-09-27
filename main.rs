// integer 100만 ~ 500만까지 싱글스레드와 멀티스레드로 각각 짜고 성능비교

use std::io::{self, Write}; // write!(io::stdout(), "{}", abc); writeln! 등 엔터 자동으로 넣는 출력문이 아니라면 io::stdout().flush() 필요
use std::sync::{Arc, Mutex};
use std::thread; // 멀티스레드용
use std::time::SystemTime; // 시간측정용 // 공유자원(뮤텍스)용

// 싱글스레드로 연산하기
fn single() {
    writeln!(io::stdout(), "100만부터 500만까지 싱글스레드로 덧셈합니다.").unwrap();
    let start = SystemTime::now(); // 시간체크

    let mut sum: u128 = 0;

    for i in 1000000..5000001 {
        sum += i;
    }

    let end = SystemTime::now().duration_since(start).unwrap().as_millis(); // 완료 후 시간체크 및 얼마나 걸렸는지 확인
    writeln!(io::stdout(), "연산완료 : {}\n걸린시간 : {}ms", &sum, &end).unwrap();
}

// 멀티스레드로 연산하기 - 첫번째 방식 : 4개의 스레드가 100만개씩 나눠서 각자 연산 후 한번에 합치기
fn multi_1() {
    writeln!(
        io::stdout(),
        "100만부터 500만까지 4개의 멀티스레드로 덧셈합니다."
    )
    .unwrap();
    let start = SystemTime::now(); // 시간체크

    let mut sum: u128 = 0;

    // 4개의 멀티스레드 반환값 담을 벡터 생성
    let mut handles = vec![];

    for i in 1..5 {
        let handle = thread::spawn(move || {
            // 범위할당
            let start_num: u128 = i * 1000000;
            let end_num: u128 = start_num + 1000000;

            let mut partial_sum = 0;
            for j in start_num..end_num {
                partial_sum += j;
            }
            partial_sum
        });
        handles.push(handle); // 스레드 변수를 밖으로 빼서 반환값에 접근할 수 있도록 하기
    }

    for handle in handles {
        sum += handle.join().unwrap(); // main 스레드는 여기서 하위 스레드들이 작업을 완료할 때까지 halt
    }
    sum += 5000000;

    let end = SystemTime::now().duration_since(start).unwrap().as_millis(); // 완료 후 시간체크 및 얼마나 걸렸는지 확인
    writeln!(io::stdout(), "연산완료 : {}\n걸린시간 : {}ms", &sum, &end).unwrap();
}

// 멀티스레드로 연산하기 - 두번째 방식 : 8개의 스레드가 50만개씩 연산 후 합치기
fn multi_2() {
    writeln!(
        io::stdout(),
        "100만부터 500만까지 8개의 멀티스레드로 덧셈합니다."
    )
    .unwrap();
    let start = SystemTime::now(); // 시간체크

    let mut sum: u128 = 0;

    // 8개의 멀티스레드 반환값 담을 벡터 생성
    let mut handles = vec![];

    for i in 2..10 {
        let handle = thread::spawn(move || {
            // 범위할당
            let start_num: u128 = i * 500000;
            let end_num: u128 = start_num + 500000;

            let mut partial_sum = 0;
            for j in start_num..end_num {
                partial_sum += j;
            }
            partial_sum
        });
        handles.push(handle); // 스레드 변수를 밖으로 빼서 반환값에 접근할 수 있도록 하기
    }

    for handle in handles {
        sum += handle.join().unwrap(); // main 스레드는 여기서 하위 스레드들이 작업을 완료할 때까지 halt
    }
    sum += 5000000;

    let end = SystemTime::now().duration_since(start).unwrap().as_millis(); // 완료 후 시간체크 및 얼마나 걸렸는지 확인
    writeln!(io::stdout(), "연산완료 : {}\n걸린시간 : {}ms", &sum, &end).unwrap();
}

// 멀티스레드로 계산하기 - 세번째 방식 : 8개의 스레드가 정답값 공유자원(뮤텍스)에 1만개씩 덧셈한 결과를 넣는 것을 반복
fn multi_3() {
    writeln!(
        io::stdout(),
        "100만부터 500만까지 8개의 멀티스레드와 뮤텍스로 덧셈합니다."
    )
    .unwrap();
    let start = SystemTime::now(); // 시간체크

    // 공유자원은 Arc<Mutex>> 타입으로 선언(여기선 공간할당만)
    let sum: Arc<Mutex<u128>> = Arc::new(Mutex::new(0));

    // 8개의 멀티스레드 담을 벡터 생성(여기선 반환값은 사용하지 않음; main스레드가 먼저 죽는것을 막는 용도로만 사용)
    let mut handles = vec![];

    for i in 2..10 {
       
       // 공유자원 접근용 변수 : 
       let sum_clone = Arc::clone(&sum);

        let handle = thread::spawn(move || {
            // 범위할당
            let start_num: u128 = i * 500000;

            // 부분연산
            let mut partial_sum = 0;

            for j in 0..50 {
                let iter_start_num = start_num + (j * 10000);
                for k in iter_start_num..iter_start_num + 10000 {
                    partial_sum += k;
                }
            }

            // 뮤텍스 락 획득시도, 변수에 접근하여 수정
            let mut data = sum_clone.lock().unwrap();
            *data += partial_sum;

            // rust에선 뮤텍스의 명시적인 해제가 필요없음 : 수명규칙에 따라 스코프 벗어날 시 자동해제, 단 변수할당 해제를 직접 명시해도 무방
            drop(data);
        });
        handles.push(handle); // 스레드 변수를 밖으로 빼기(반환값은 딱히 사용안함, 스레드 수명관리용)
    }

    for handle in handles {
        handle.join().unwrap(); // main 스레드는 여기서 하위 스레드들이 작업을 완료할 때까지 halt
    }

    // 뮤텍스 락 해제 후 변수값 받아오기
    let final_value = *sum.lock().unwrap() + 5000000;

    let end = SystemTime::now().duration_since(start).unwrap().as_millis(); // 완료 후 시간체크 및 얼마나 걸렸는지 확인
    writeln!(
        io::stdout(), "연산완료 : {}\n걸린시간 : {}ms", &final_value, &end).unwrap();
}

fn main() {
    single();

    println!();

    multi_1();

    println!();

    multi_2();

    println!();

    multi_3();
}
