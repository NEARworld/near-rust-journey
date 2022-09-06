// constants can be declared in global scope
const MY_GLOBAL_COSNTANT: i32 = 100;
// variables cannot be declared in global scope
let my_global_let = 200;

fn main() {
    // variables are immutable by default in rust
    let x = 10;
    x = 100; // impossible to assign a value to immutable variables

    let mut x = 10; // Shadowing a variable
    x = 100; // possible to assign a value to mutable variables

    const my_constant: i32 = 10; // warning. Constant should be uppercases
    const TEN = 10; // Error. Constant should have a type annotation
    const TEN: i32 = 10; // Correct

    // Variables are evaluated at runtime after compile time
    let x = 10;
    let y = 10;
    // Error. Constant value is evaluated at compile time.
    // But Constant value is arithmetic operation with variables that will be evaluated at runtime
    const SUM: i32 = x + y;
}