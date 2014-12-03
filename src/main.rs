extern crate spell_corrector;

#[cfg(not(test))]
fn main() {
    use spell_corrector::NwordCorrector;
    use std::os;
    let corrector = NwordCorrector::new();
    let args = os::args();
    println!("{} -> {}", args[1], corrector.correct(args[1].clone()));
}
