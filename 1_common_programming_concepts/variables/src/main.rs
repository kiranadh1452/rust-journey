const DATA: i32 = 5;
// let test : i32 = 5; // This will fail to compile as let variables can't be declared in global scope

fn main() {
    /*
     Mutable and Non-mutable variables
        1. Variables are immutable by default
        2. Use the mut keyword to make them mutable
        3. Shadowing is allowed
    */
    {
        let immutable_var = 5;
        println!("The value of immutableVar is: {}", immutable_var);

        // immutable_var = 6; // This will fail to compile as immutable_var is immutable

        // creating a mutable variable
        let mut mutable_var = 5;
        println!("The value of mutableVar is: {}", mutable_var);

        mutable_var = 6; // This will compile
        println!("The value of mutableVar is: {}", mutable_var);
    }

    /*
    Constants
        1. Constants are always immutable
        2. Constants are declared using the const keyword
        3. Calls in constants are limited to constant functions, tuple structs and tuple variants
        4. Constants can be declared in any scope (even outside the main function) wheras let variables can't.
    */
    {
        const FULL_MARKS: i32 = 100;
        // const FULL_MARKS_2 = 100; // This won't compile as the type is not specified for FULL_MARKS_2. It is mandatory to specify the type for constants

        println!("The Full Marks is : {}", FULL_MARKS);

        // return value from a constant function is acceptable in a constant
        const VALUE_READ: i32 = multiply_by_ten(12);
        println!("The value read is: {}", VALUE_READ);

        // return value from a non-constant function is not acceptable in a constant
        // const VALUE_READ_2: i32 = multiply_by_ten_v2(12); // This will fail to compile
    }

    /*
        Shadowing
            1. Shadowing is allowed in Rust
            2. Shadowing is different from mutability. We are not making a variable mutable using shadowing, we are re-declaring it.
                It also allows us to change the type of the variable also.
            3. Shadowing is done by using the let keyword
    */
    {
        let my_name = "Kiran Adhikari";
        println!("My name is: {}", my_name);

        let my_name = my_name.len(); // Shadowing the variable my_name
        println!("My name has {} characters", my_name);
    }
}

// Helper functions
const fn multiply_by_ten(value: i32) -> i32 {
    /*
       ``const RESULT: i32 = 10 * value`` will fail to compile as const functions can't have statements that are not constant.
       Here the variable `value` is non-constant
    */
    // const RESULT: i32 = 10 * value;
    let result = 10 * value;
    return result;
}

// You will get warning for this function if it is not used anywhere
fn multiply_by_ten_v2(value: i32) -> i32 {
    return 10 * value;
}
