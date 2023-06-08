fn main() {
    /*
        Static Types:
            - Integer
            - Float
            - Boolean
            - Character
    */
    {
        // Integers and Floats
        {
            // All possible Signed Integer types
            {
                let _int8: i8 = 127;
                let _int16: i16 = 32767;
                let _int32: i32 = 2147483647; // this is the default type for integer numbers
                let _int64: i64 = 9223372036854775807;
                let _int128: i128 = 170141183460469231731687303715884105727;
                let _int: isize = 9223372036854775807;

                // All possible Unsigned Integer types
                let _uint8: u8 = 255;
                let _uint16: u16 = 65535;
                let _uint32: u32 = 4294967295;
                let _uint64: u64 = 18446744073709551615;
                let _uint128: u128 = 340282366920938463463374607431768211455;
                let _uint: usize = 18446744073709551615;

                /*
                    What is `isize` and `usize`?
                    ----------------------------
                    The isize type in Rust is used to represent a signed integer that has the same size as a pointer on the current platform.
                    It is architecture-dependent and will have a size of 32 bits on 32-bit platforms and 64 bits on 64-bit platforms.

                    Integer Overflow
                    ----------------
                    https://rust-book.cs.brown.edu/ch03-02-data-types.html#integer-overflow

                    Integer overflow aren't checked in --release mode. What would happen if we add 1 to the maximum value of i8?
                    It would wrap around to the minimum value of i8. This is called two's complement wrapping.
                */
            }

            // All possible Float types
            {
                let _float32: f32 = 3.40282347e+38;
                let _float64: f64 = 1.7976931348623157e+308; // this is the default type for floating-point numbers
            }

            // Interaction between float and integers
            {
                let _marks: [i32; 5] = [100, 90, 80, 70, 60];
                let _total: i32 = _marks.iter().sum();

                // let _average: f64 = _total / _marks.len();
                // Above line will throw an error because the result of the division is an integer and we are trying to assign it to a float variable

                let _average: f64 = _total as f64 / _marks.len() as f64;
                println!("Sum : {} \n Average: {}", _total, _average);
            }

            // Numeric Operations
            {
                // See how the types are inferred
                let _sum = 5 + 10;
                let _difference = 95.5 - 4.3;
                let _product = 4 * 30;
                let _quotient = 56.7 / 32.2;
                let _remainder = 43 % 5;
            }
        }

        // Boolean
        {
            let _t = true;
            let _f: bool = false; // with explicit type annotation
        }

        /*
            Character
            ---------
           They are specified with single quotes
           Unlike other languages like C/C++ where character is of 1 byte, character in rust is of 4 bytes.
           It is a Unicode Scalar Value and hence can represent any character in the world.
        */
        {
            let _c = 'z';
            let _z = 'ℤ';
            let _heart_eyed_cat = '😻';

            // The following is not a character.
            // let _not_a_char: char = "a"; // Use single quotes for characters and double quotes for strings
        }
    }

    /*
        Compound Types
            - Tuple
            - Array
    */
    {
        /*  Tuple
            -----

            Can group multiple values of different types
            Fixed length: Once declared, they cannot grow or shrink in size
        */
        {
            let _tup1 = ("Kiran Adhikari", 23, 70);

            // Destructuring a tuple
            let (_name, _age, _height_in_inches) = _tup1;

            // Accessing elements
            let _name = _tup1.0;
            let _age = _tup1.1;
            let _height_in_inches = _tup1.2;

            // _tup1.1 = 24; // This will throw an error because tuples are immutable by default
        }

        /*
            Array
            -----

            Can group multiple values of same type
            Fixed length: Once declared, they cannot grow or shrink in size
            Arrays are useful when you want your data allocated on the stack rather than the heap
        */
        {
            let _arr1 = [1, 2, 3, 4, 5];
            let _arr2: [i32; 5] = [1, 2, 3, 4, 5]; // with explicit type annotation
            let _arr3 = [3; 5]; // [3, 3, 3, 3, 3]

            // Accessing elements
            let _first = _arr1[0];
            let _second = _arr1[1];

            // _arr1[0] = 10; // This will throw an error because arrays are immutable by default

            // Accessing elements beyond bound
            let _out_of_bound = _arr1[10];

            /*
                Rust Memory Safety in this case
                -------------------------------

                In rust, accessing elements beyond bound is a runtime error.
                You would get a runtime error like this: thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10'
                Other languages like C/C++ do not perform bounds checking by default.
                As a result, accessing elements outside the bounds of an array in C or C++ can lead to undefined behavior,
                such as reading or writing to unintended memory locations, causing crashes, data corruption, or security vulnerabilities.

                In other languages, invalid memory access might be provided for cases as above.
                Rust protects you against this kind of error by immediately exiting.
                The program will be safely aborted, which is much better than continuing with undefined behavior.
            */
        }
    }
}
