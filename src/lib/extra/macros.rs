// macros refer to declarative macros with macro_rules!
// and three kind of procedural macros
//
// #[derive] macros that specify code added with derive attribute
// attribute-like macros that define curstom attributes usable on any item
// function like macros that look like function calls but operate on tokens specified as their
// argument

// macros - way of writing code that writes other code
// metaprogramming
//
// function signature must declare number and type of params
// macros can take variable numver of params
//
// macros are expanded before compile time,
//
// macros more complex than funcs
//
//
// funcs call anywhere and define anywhere
// macros define/bring scope before calling them

// declartative macros macro_rules!

#[macro_export]
macro_rules! myvec {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn use_my() {
    let a = myvec![1, 2, 3];
}

// procedural macros, generating code from attributes
// defn's must reside in their own crate
// with proc-macro crate type

// custom derive macros
// #[derive(Debug)]
// attribute-like macros
// #[route(GET, "/")]
// function like macros
// sql!(SELECT * FROM users)
