#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]
#![warn(private_doc_tests)]

//! This module demonstrates Clippy lints and Cargo doc warnings.
/// A function with missing documentation.
fn main() {
    let x = 42; // Unused variable (warns with `clippy::let_and_return` if returned)
    let y = x + 0; // Redundant operation (`clippy::identity_op`)
    let z = y * 1; // Redundant operation (`clippy::identity_op`)
    println!("z: {}", z);

    let vec = vec![1, 2, 3, 4, 5];
    for i in 0..vec.len() {
        // Better to use `.iter()` (`clippy::needless_range_loop`)
        println!("{}", vec[i]);
    }

    if x == 42 {
        println!("x is 42");
    } else if x == 42 {
        // This branch is redundant (`clippy::collapsible_if`)
        println!("x is still 42");
    }

    // Shadow for testing
    let _unused_result = "Hello".to_string(); // Unused result (`clippy::unused_self`)
}

/// This function is undocumented but contains a broken link.
/// See [`NonExistentStruct`] for more details.
fn undocumented_function() {
    // This function is missing documentation and contains an invalid intra-doc link.
}
