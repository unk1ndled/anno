#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]
#![warn(private_doc_tests)]

//! This module demonstrates various Clippy lints and documentation warnings.

/// A function with missing documentation that triggers Clippy warnings.
fn main() {
    let a = 10; // Unused variable (warns with `clippy::let_and_return` if returned)
    let b = a * 0; // Always zero (`clippy::identity_op`)
    let c = b + 5 - 5; // Redundant calculation (`clippy::identity_op`)
    println!("c: {}", c);

    let numbers = vec![10, 20, 30, 40];
    for i in 0..numbers.len() {
        // Suggests using `.iter()` (`clippy::needless_range_loop`)
        println!("{}", numbers[i]);
    }

    if a > 5 {
        if a > 5 {
            // Suggests collapsing (`clippy::collapsible_if`)
            println!("a is greater than 5");
        }
    }

    // Shadowing an unused result
    let _ignored = "World".to_string(); // Unused result (`clippy::unused_self`)
}

/// This function contains a broken intra-doc link.
/// See [`UndefinedType`] for more details.
fn another_undocumented_function() {
    // Missing documentation and invalid intra-doc link.
}
