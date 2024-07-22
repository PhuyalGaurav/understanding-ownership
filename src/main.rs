fn main() {
    scopedropping();
}

fn scopedropping() {
    {
        let name: String = String::from("Gaurav in the main scope of function: scopedropping()");

        // name is allocated using heap & dropped when the scope ends
        println!("{name}")
    }
    // scope ends name is no longer valid
    // sprintln!("{name}")
}
