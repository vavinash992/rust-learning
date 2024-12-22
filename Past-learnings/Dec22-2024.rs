// fn main(){
//     let mut s = String::from("hello");

//     let r1 = &mut s;
//     let r2 = &s;

//     println!("{}, {}", r1, r2);
// }

fn main() {
    let mut s = String::from("hello");

    change(&mut s);
    println!("{s}")
}

fn change(some_string: &mut String) {
    some_string = some_string.copy();
    some_string.push_str(", world");
}