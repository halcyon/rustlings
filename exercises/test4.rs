// test4.rs
// This test covers the sections:
// - Modules
// - Macros

// Write a macro that passes the test! No hints this time, you can do it!


macro_rules! my_macro {
    // () => {
    //     println!("Check out my macro!");
    // };

    ($val:expr) => {
        // println!("Look at this other macro: {}", $val);
        "Beef"
    }
}

fn main() {
    if my_macro!("world!") != "Hello world!" {
        panic!("Oh no! Wrong output!");
    }
}
