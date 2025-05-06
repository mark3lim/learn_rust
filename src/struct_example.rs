// Learning Structs in Rust
pub mod user_mod{
    use std::io::{self, Write}; //중첩 경로를 사용해서 패키지 가져오기.

    #[derive(Debug)]
    struct User {
        id: u32,
        name: String,
        email: String,
        phone_num: String,

    }

    impl User {
        fn new(id: u32, name: String, email: String, phone_num: String) -> User {
            User {
                id,
                name,
                email,
                phone_num,
            }
        }

        fn empty_user() -> User {
            User {
                id: 1,
                name: String::new(),
                email: String::new(),
                phone_num: String::new(),
            }
        }

        fn delete_user(user: User) {
            // The function takes ownership of the user object.
            // Automatically drops the user form the memory.
            println!("Deleting User: {}", user.id);
        }

        fn update_email(user: &mut User, new_email: String) {
            user.email = new_email;
            println!("Updated email for User {}: {}", user.id, user.email);
        }

        fn update_phone_num(user: &mut User, new_phone_num: String) {
            user.phone_num = new_phone_num;
            println!("Updated phone number for User {}: {}", user.id, user.phone_num);
        }
    }

    pub(crate) fn test() {
        println!("Insert a new User:");
        let mut user = User::empty_user();

        user.name = get_input("User name: ");
        user.email = get_input("User email: ");
        user.phone_num = get_input("User phone num: ");

        println!("[info] User: {:?}", user);
    }

    fn get_input(prompt: &str) -> String {
        let mut input = String::new();
        print!("{}", prompt);
        io::stdout().flush().expect("Failed to flush stdout"); //print!()는 바로 출력하는 것이 아니라 버퍼에 저장되기 때문에 강제로 버퍼에 있는 내용을 출력한다.
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.trim().to_string()
    }
}