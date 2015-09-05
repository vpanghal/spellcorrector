extern crate spell_core;

use spell_core::NwordCorrector;

#[test]
fn test_correction() {
    let corrector = NwordCorrector::new();
    check_equal("spelling", "speling", &corrector);
    check_equal("correct", "korrect", &corrector);
    check_equal("surrounding", "serounding", &corrector);
    check_equal("separation", "seperation", &corrector);
    check_equal("progression", "progresion", &corrector);
}

fn check_equal(correct : &str, given : &str, corrector : & NwordCorrector) {
    assert_eq!(correct.to_string(), corrector.correct(given.to_string()));
}
