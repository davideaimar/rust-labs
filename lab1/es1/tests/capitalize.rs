use capitalizer::capitalize;

#[test]
fn more_words() {
    assert_eq!(capitalize("ciao come va"), "Ciao Come Va");
}

#[test]
fn one_word() {
    assert_eq!(capitalize("ciao"), "Ciao");
}

#[test]
fn begin_with_accented() {
    assert_eq!(capitalize("èllo àorld"), "Èllo Àorld");
}

#[test]
fn empty_string() {
    assert_eq!(capitalize(""), "");
}

#[test]
fn more_spaces() {
    assert_eq!(capitalize("ciao       mondo"), "Ciao       Mondo");
}

#[test]
fn german_char() {
    assert_eq!(capitalize("ßprova"), "SSprova");
}