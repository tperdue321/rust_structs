// structs project
// exploring https://doc.rust-lang.org/book/second-edition/ch05-00-structs.html
// author: Travis Perdue
// purpose is learn Rust structs by writing the code and leave a project I can
// reference in the future
struct User {
    id: i32,
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
} 

// tuple structs for when you want to give description to a tuple but
// key: value would be overly verbose
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct Unit_Like_Struct {}
fn main() {

    // order of key: value is not important
    // in order to manipulate any field
    // the entire struct must be mutable
    let mut user = User {
        id: 1,
        username: String::from("someone"),
        sign_in_count: 1,
        active: true,
        email: String::from("someone@somewhere.com"),
    };
    // can change mutable fields
    user.email = String::from("someone_else@somewhere_else.com");

    let user2 = build_user( String::from("someName"),
                            String::from("some@email.com"),
                            user.id);

    // can use repeated struct values from a previous struct in creating a new
    // struct
    let user3 = User {
        id: user2.id + 1,
        // will have all the same attributes as user2 except for the id
        ..user2
    };

    let purple = Color(230, 230, 250);
    let origin = Point(0, 0, 0);
    // if a function calc_point(point: Point)
    // takes a Point struct as a param
    // then it cannot take a Color struct even though the datatypes
    // and size are the same
    // calc_point(purple) // BAD
}

fn build_user(username: String, email: String, id: i32) -> User {

    User {
        // id field is for example and fun. not being used
        // like ids normally are.
        id: id + 1,
        // if the value param and the key are the same can be written:
        username,
        // instead of username: username,
        sign_in_count: 1,
        active: true,
        email,
    }
}
