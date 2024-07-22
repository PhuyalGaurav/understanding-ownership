fn main() {
    scopedropping();
    string_copying()
}

fn scopedropping() {
    {
        // name is allocated using heap & dropped when the scope ends
        let name: String = String::from("Gaurav in the main scope of function: scopedropping()");

        let nameptr: *const u8 = name.as_ptr();
        println!("{:?}", nameptr);
    }
    // scope ends name is no longer valid
    // sprintln!("{name}")
}

fn string_copying() {
    let s1: String = String::from("hello");
    let s2: String = s1;

    // s1 is no longer valid, so that the double free error doesnot occur when s1 ands2 goes out of scope
    // println!("{s1}, world!");
    println!("{s2}, world!");

    let s1: String = s2.clone();

    // Both of them are now valid as s1 is a Deep copy of s2

    println!("String 1 = {s1}, String 2 = {s2}");
}
