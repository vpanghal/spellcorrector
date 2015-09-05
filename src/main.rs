extern crate spell_core;

#[cfg(not(test))]
fn main() {
    use std::env;
    use spell_core::NwordCorrector;
    let corrector = NwordCorrector::new();
    let args : Vec<String> = env::args().collect();
    println!("{} -> {}", args[1], corrector.correct(args[1].clone()));
}
