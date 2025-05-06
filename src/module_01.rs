// 외부엣 접근 가능
pub mod home_mod {
    // 외부에 접근 가능
    pub mod main_mod {
        // 외부에서 사용 가능
        pub fn first_mod() {
            println!("[home_mod] first_mod");
            super::private_mod::third_mod(); // private_mod의 third_mod를 사용
        }
    }
    //외부에서 접근 가능
    pub mod sub_mod {
        // 외부에서 사용 불가
        fn second_mod() {
            println!("[home_mod] second_mod");
        }
    }
    //외부에서 접근 불가
    mod private_mod {
        //외부에서 사용 불가(mod가 private), 모듈에서는 사용 가능
        pub fn third_mod() {
            println!("[home_mod] third_mod");
        }
    }
    // 외부에서 접근 불가
    mod private_mod_2 {
        // 외부에서 사용 불가
        fn fourth_mod() {
            println!("[home_mod] fourth_mod");
        }
    }
}