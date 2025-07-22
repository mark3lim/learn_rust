// Main code for learning Rust project
// run any kind of code to see the sample output
mod guess_game; // 무작위 숫자 맞추기 게임
mod use_db; //DB 연결 예제
mod struct_example; // 구조체 예제
mod module_01; // 모듈 예제 01

mod string_fn;

use std::io;
// 문자열 연습용
use string_fn::string_size;

mod collectors;
use collectors::vec_01::vec_01::vec_fn;
use collectors::q1_list::q1_list::q1_fn;
use collectors::q2_string::q2_string::test as q2_test;
use collectors::q3_hashmap::q3_hashmap::test as q3_test;

use struct_example::user_mod::test as mod_test;

fn main() {
    let mut input = String::new();

    println!("Input:");

    // 표준 입력(키보드)으로부터 한 줄을 읽어옵니다.
    // read_line은 Result를 반환하므로, 에러 발생 시 expect 메시지가 출력됩니다.
    io::stdin().read_line(&mut input)
        .expect("입력 줄을 읽는데 실패했습니다.");

    // read_line으로 읽은 문자열은 끝에 줄바꿈 문자(\n)가 포함되어 있습니다.
    // trim() 메소드를 사용하여 앞뒤의 공백 및 줄바꿈 문자를 제거합니다.
    let input = input.trim();

    // guess_game::start_game();
    // test();
    // module_01::home_mod::main_mod::first_mod();
    // q1_fn();
    // q2_test();
    // q3_test();
    // string_size::get_len(input.to_string());
    string_size::get_word_len(input.to_string());
}