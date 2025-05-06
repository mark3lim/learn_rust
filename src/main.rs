// Main code for learning Rust project
// run any kind of code to see the sample output
mod guess_game; // 무작위 숫자 맞추기 게임
mod use_db; //DB 연결 예제
mod struct_example; // 구조체 예제
mod module_01; // 모듈 예제 01

mod collectors;
use collectors::vec_01::vec_01::vec_fn;
use collectors::q1_list::q1_list::q1_fn;
use collectors::q2_string::q2_string::test as q2_test;
use collectors::q3_hashmap::q3_hashmap::test as q3_test;

use struct_example::user_mod::test as mod_test;

fn main() {
    // guess_game::start_game();
    // test();
    // module_01::home_mod::main_mod::first_mod();
    // q1_fn();
    // q2_test();
    q3_test();
}