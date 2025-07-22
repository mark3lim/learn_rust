pub mod user_info {
//! This module is about manufacturing User info.
//!
//! Main functions:
//! - create user
//! - check user info data is suitable
//! - update user info
//! - delete user info

    use std::any::type_name;

    struct UserInfo {
        category: String,
        name: char,
    }

    struct User {
        id: String,
        user_id: String,
        email: String,
        user_type: UserType,
    }

    impl User {
        fn new() -> User {
            User {
                id: "100001".to_string(),
                user_id: String::new(),
                email: String::new(),
                user_type: UserType::new(),
            }
        }

        fn  new(id: String, user_id: String, email: String, user_type: UserType) -> User {
            User {
                id,
                user_id,
                email,
                user_type,
            }
        }

        /// Checks user user_id and email is suitable.
        /// usr_id must be "String" type and email has to be like 'example@mail.com'.
        fn check_value(user: &User) -> bool {
            user = user.clone();
            let mut is_okay =  false;

            if "String" == return_type(&user.user_id) { is_okay = true; }
            if "String" == return_type(&user.email) { is_okay = true; }

            //check email
            user_email = user.email.to_string();
            let email_regex = Regex::new(
                r"(?i)^[a-z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-z0-9-]+(?:\.[a-z0-9-]+)*$"
            ).unwrap();
            is_okay = email_regex.is_match(user_email);

            return is_okay
        }
    }

    impl UserType {
        fn new()  -> UserInfo {
            UserInfo {
                category: "user".to_string(),
                name: 'N',
            }
        }

    }

    pub fn print_user(user: &User) {
        println!("==============================");
        println!("User ID:\t\t{}", user.id);
        println!("User's ID:\t\t{}", user.user_id);
        println!("User email:\t\t{}", user.email);
        println!("User type:\t\t{}", user.user_type.get_type());
        println!("==============================");
    }

    /// Returns the type in shorter reference.
    /// Data type "alloc::string::String" returns as "String".
    ///
    /// # Example:
    /// ```
    /// models::User -> User
    /// alloc::string::String -> String
    /// ````
    fn return_type<T>(_: &T) ->  &'static str {
        let full_type = type_name::<T>();
        if full == "alloc::string::String" {
            "String"
        } else {
            // 마지막 "::" 뒤만 잘라서 반환 (ex: models::User → User)
            full.rsplit("::").next().unwrap_or(full)
        }
    }

}
