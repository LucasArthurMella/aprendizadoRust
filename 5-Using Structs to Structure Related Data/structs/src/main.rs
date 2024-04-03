// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64
// }


// fn main() {

//     let mut user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count: 1
//     };

//     user1.email = String::from("anotheremail@example.com");
    
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username: username,
//         email: email,
//         sign_in_count: 1,
//     }
// }

// fn build_user(email: String, username:String) -> User {
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1
//     }
// }

// fn main(){
//     let user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count: 1
//     };

//     // let user2 = User {
//     //     active: user1.active,
//     //     username: user1.username,
//     //     email: String::from("another@example.com"),
//     //     sign_in_count: user1.sign_in_count
//     // };

//     let user2 = User {
//         email: String::from("another@example.com"),
//         ..user1
//     };
// }

// fn main(){
//     struct Color(i32, i32, i32);
//     struct Point(i32, i32, i32);

//     let black = Color(0,0,0);
//     let origin = Point(0,0,0);


// }

// fn main(){
//     struct AlwaysEqual;

//     fn main(){
//         let subject = AlwaysEqual;
//     }
// }

// struct User2 {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64
// }

// fn main(){
//     let user1 = User {
//         active: true,
//         username: "someusername123",
//         email: "someone@xample.com",
//         sign_in_count: 1
//     }
// }