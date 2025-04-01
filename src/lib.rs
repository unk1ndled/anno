#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]
#![warn(private_doc_tests)]

//! This module demonstrates Clippy lints and Cargo doc warnings.

/// A function with missing documentation.
fn main() {
    let num = 100; // Unused variable (warns with `clippy::let_and_return` if returned)
    let sum = num + 0; // Redundant operation (`clippy::identity_op`)
    let product = sum * 1; // Redundant operation (`clippy::identity_op`)
    println!("product: {}", product);

    let list = vec![10, 20, 30, 40, 50];
    for i in 0..list.len() {
        // Better to use `.iter()` (`clippy::needless_range_loop`)
        println!("{}", list[i]);
    }

    if num == 100 {
        println!("num is 100");
    } else if num == 100 {
        // This branch is redundant (`clippy::collapsible_if`)
        println!("num is still 100");
    }

    // Shadow for testing
    let _unused_string = "Rust".to_string(); // Unused result (`clippy::unused_self`)
}

/// This function is undocumented but contains a broken link.
/// See [`UnknownType`] for more details.
fn undocumented_fn() {
    // This function is missing documentation and contains an invalid intra-doc link.
}
