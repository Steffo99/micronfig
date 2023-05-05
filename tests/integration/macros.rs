use std::env;

micronfig::required!(PLAYER_NAME, String);
micronfig::required!(PLAYER_ID, u64);
micronfig::optional!(IS_SUS, bool);


#[test]
fn test_macros() {
    env::set_var("PLAYER_NAME", "Steffo");
    env::set_var("PLAYER_ID", "1234");
    env::remove_var("IS_SUS");

    assert_eq!(*PLAYER_NAME, "Steffo");
    assert_eq!(*PLAYER_ID, 1234u64);
    assert_eq!(*IS_SUS, None);
}
