# Kata: Karate Chop

## Challenge

Standard acronym solver.

Input is a dictionary with 1 word per line.

## Test Data

```rust

    #[test]
    fn it_returns_nothing_for_a_empty_initial_letters_and_dictionary() {
        let result: Vec<String> = Vec::new();
        assert_eq!(solve_acronym(&mut "".as_bytes(), "".to_string()), result);
    }
    #[test]
    fn it_returns_nothing_for_a_empty_initial_letters() {
        let result: Vec<String> = Vec::new();
        assert_eq!(solve_acronym(&mut "testing".as_bytes(), "".to_string()), result);
    }
    #[test]
    fn it_returns_nothing_for_a_empty_dictionary() {
        let result: Vec<String> = Vec::new();
        assert_eq!(solve_acronym(&mut "".as_bytes(), "testing".to_string()), result);
    }
    #[test]
    fn testing_matches_testing() {
        let mut  result: Vec<String> = Vec::new();
        result.push("testing".to_string());
        assert_eq!(solve_acronym(&mut "testing".as_bytes(), "testing".to_string()), result);
    }
    #[test]
    fn act_matches_cat_act_tac() {
        let mut  result: Vec<String> = Vec::new();
        result.push("act".to_string());
        result.push("cat".to_string());
        result.push("tac".to_string());
        assert_eq!(solve_acronym(&mut "act\ncat\ntac".as_bytes(), "cat".to_string()), result);
    }
```
 
# Running

Format code

```
rustup run nightly cargo fmt --all --
```

Run tests

```
$ rustup run stable cargo test               master ✚ ◼
      Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
       Running target/debug/deps/kata_anacronym-466783b986774df3
  
  running 6 tests
  test tests::it_returns_nothing_for_a_empty_initial_letters_and_dictionary ... ok
  test tests::it_returns_nothing_for_a_empty_initial_letters ... ok
  test tests::it_sorts_the_letters_alphabetically ... ok
  test tests::act_matches_cat_act_tac ... ok
  test tests::it_returns_nothing_for_a_empty_dictionary ... ok
  test tests::testing_matches_testing ... ok
  
  test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  
     Doc-tests kata-anacronym
  
  running 0 tests
  
  test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
  

```
