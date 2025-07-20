fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    // let a = ???

    let a = r#"Getting started
If you're just getting started with Rust and would like a more detailed walk-through, see our getting started page.

Toolchain management with rustup
Rust is installed and managed by the rustup tool. Rust has a 6-week rapid release process and supports a great number of platforms, so there are many builds of Rust available at any time. rustup manages these builds in a consistent way on every platform that Rust supports, enabling installation of Rust from the beta and nightly release channels as well as support for additional cross-compilation targets.

If you've installed rustup in the past, you can update your installation by running rustup update.

For more information see the rustup documentation."#;
    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
