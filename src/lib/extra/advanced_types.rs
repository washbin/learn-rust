fn type_aliases() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
}

// Never type
// ! -> empty type in type theory
//
// function that return never are called diverging functions
//
// ! can be coerced to any type
fn match_must_return_same_type() {}
// continue, panic, loop

// Dynamically Sized Types and Sized Trait
// DST -> dynamically sized types / unsized types

fn dst_cant_be_used_directly_as_variable_or_argument() {
    // let s1: str = "Hello";
    let s1: &str = "Hello";

    // &T is a single value,
    // stores the memory of address of where T is stored

    // &str is two values,
    // one is a pointer to the string, the other is the length of the string
    //
    // size of &str known at compile time, 2 * usize

    // Sized trait
    // fn generic<T>(t: T) is actually fn generic<T: Sized>(t: T)
    // to used dst, we use fn generic(T: ?Sized)(t: &T)
    // ?Trait only available for Sized
}

//
// 1015750919046001
// 1015750919046001
