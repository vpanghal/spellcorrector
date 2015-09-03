extern crate spell_corrector;

#[cfg(not(test))]
fn main() {
    use std::env;
    use spell_corrector::NwordCorrector;
    let corrector = NwordCorrector::new();
    let args : Vec<String> = env::args().collect();
    println!("{} -> {}", args[1], corrector.correct(args[1].clone()));
}
