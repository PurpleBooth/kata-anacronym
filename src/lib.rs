use std::io::BufRead;

fn solve_acronym(dictionary: &mut std::io::BufRead, initial_letters: String) -> Vec<String> {
    let mut found_acronyms: Vec<String> = Vec::new();
    let hashed_initial_word = hash_word(initial_letters);

    for wrapped_line in dictionary.lines() {
        let line = wrapped_line.unwrap();

        if hash_word(line.clone()) == hashed_initial_word {
            found_acronyms.push(line);
        }
    }

    found_acronyms
}

fn hash_word(word: String) -> String {
    let normalised_word = word.to_lowercase();
    let mut split_word = normalised_word.split("").collect::<Vec<&str>>();
    split_word.sort();
    split_word.join("")
}

#[cfg(test)]
mod tests {
    use hash_word;
    use solve_acronym;

    #[test]
    fn it_sorts_the_letters_alphabetically() {
        assert_eq!(hash_word("cat".to_string()), "act");
    }

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
}
