fn main() {
    scopedropping();
    string_copying();
    taking_ownership();
    taking_ownership_and_giving();
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

fn taking_ownership() {
    let my_name: String = String::from("Gaurav being owned by own()");
    own(my_name);

    // my name will no longer be valid since thw own() function has taken over its ownership.
    //println!("{my_name}") -> So this will not work
    fn own(name: String) {
        println!("{name}")
    }
}

fn taking_ownership_and_giving() {
    let my_name: &mut String = &mut String::from("Gaurav being owned x 2");

    // ownership of my_name is given to take_and_give() but also returned back so it is still in memory
    let my_name: &mut String = take_and_give(my_name);

    // since my_name is the owner of itself now it can be used in this scope
    my_name.push_str(" and is being used my the main scope");

    fn take_and_give(name: &mut String) -> &mut String {
        name.push_str(" But also Returned");
        println!("{name}");
        return name;
    }
}
