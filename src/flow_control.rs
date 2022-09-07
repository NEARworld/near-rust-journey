pub fn if_condition() -> i32 {

    let tmp = if true {
        3 // if expression evaluates to 3
    } // ;

    // // In Rust, only boolean values work on conditions.
    // if 1 {
    //     println!("Does this work?"); // No it doesn't
    // }

    //   |
    // 8 | if 1 {
    //   |    ^ expected `bool`, found integer

    // For more information about this error, try `rustc --explain E0308`.

    else {panic!()};
    tmp
}

pub fn else_if_condition() -> i32 {
    let condition = true;
    let x = if condition { 3 } else { 5 };

    x
}

pub fn loop_flow() -> i32 {
    // loop {
    //     println!("infinte loop");
    // }
    // ^ Output:
    // infinte loop
    // infinte loop
    // infinte loop
    // ...
    // <runs forever>

    // loop {
    //     println!("the first line inside loop");
    //     loop {
    //         continue; // move to the next iteration of the second loop
    //         println!("after continue"); // never reach here
    //     }
    // }
    // ^ Output:
    // the first line inside loop
    // <stuck in here>

    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
            // This would return the current counter value as an expression in place of the loop itself
        }
    };

    result
}

pub fn loop_label() -> bool {
    'first_loop: loop {
    println!("first loop");
        loop {
            println!("second loop");
            break 'first_loop; // this will stop the first loop because of the loop label 'first_loop
        }
    }
    
    true // Indication that it ran properly without any errors
}

pub fn while_loop() -> i32 {
    let mut i = 5;
    while i > 0 {
        i -= 1;
    }
    // Decrement i by 1, untill it's greater than 0 

    i
}

pub fn for_loop() -> bool {
    let x = [1, 2, 3, 4, 5];

    for val in x {
        println!("{val}");
    }
    // ^ Output:
    // 1
    // 2
    // 3
    // 4
    // 5

    true // Indication that it ran properly without any errors
}

// Testing all the functions
#[test]
fn test_if_condition() {
    assert_eq!(if_condition(), 3);
}

#[test]
fn test_else_if_condition() {
    assert_eq!(else_if_condition(), 3);
}

#[test]
fn test_loop_flow() {
    assert_eq!(loop_flow(), 10);
}

#[test]
fn test_loop_label() {
    assert!(loop_label());
}

#[test]
fn test_while_loop() {
    assert_eq!(while_loop(), 0);
}

#[test]
fn test_for_loop() {
    assert!(for_loop());
}