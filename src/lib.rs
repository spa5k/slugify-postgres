use pgx::*;
use slugify_rs::slugify;

pg_module_magic!();

#[pg_extern]
fn slug(text: &str) -> String {
    slugify!(text)
}

#[pg_extern]
fn slug_sep(text: &str, separator: &str) -> String {
    slugify!(text, separator = separator)
}

#[pg_extern]
fn slug_rand(text: &str) -> String {
    slugify!(text, randomness = true)
}

#[pg_extern]
fn slug_rand_sep(text: &str, separator: &str) -> String {
    slugify!(text, separator = separator, randomness = true)
}

#[pg_extern]
fn slug_rand_c(text: &str, randomness_length: i32) -> String {
    let randomess_length_as_usize = randomness_length as usize;
    slugify!(
        text,
        randomness = true,
        randomness_length = randomess_length_as_usize
    )
}

#[pg_extern]
fn slug_rand_sep_c(text: &str, separator: &str, randomness_length: i32) -> String {
    let randomess_length_as_usize = randomness_length as usize;
    slugify!(
        text,
        separator = separator,
        randomness = true,
        randomness_length = randomess_length_as_usize
    )
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_hello_world() {
        assert_eq!("hello-world", crate::slug("hello world"));
        assert_eq!("hello_world", crate::slug_sep("hello world", "_"));
        assert_eq!("helloworld", crate::slug_sep("hello world", ""));
        assert_eq!("hello%world", crate::slug_sep("hello world", "%%"));
    }

    #[pg_test]
    fn test_email() {
        assert_eq!(crate::slug("alice@bob.com"), "alice-bob-com");
        assert_eq!(crate::slug("alice@bob.com"), "alice-bob-com");
    }

    #[pg_test]
    fn test_starts_with_number() {
        assert_eq!(crate::slug("10 amazing secrets"), "10-amazing-secrets");
    }

    #[pg_test]
    fn test_contains_numbers() {
        assert_eq!(crate::slug("the 101 dalmatians"), "the-101-dalmatians");
        assert_eq!(
            crate::slug_rand("the 101 dalmatians").len(),
            "the-101-dalmatians".len() + 5
        );
        assert_eq!(
            crate::slug_rand_c("the 101 dalmatians", 10).len(),
            "the-101-dalmatians".len() + 10
        );
        assert_eq!(
            crate::slug_rand_sep_c("the 101 dalmatians", "_", 10).len(),
            "the-101-dalmatians".len() + 10
        );
    }

    #[pg_test]
    fn test_ends_with_number() {
        assert_eq!(crate::slug("lucky number 7"), "lucky-number-7");
    }

    #[pg_test]
    fn test_numbers_and_symbols() {
        assert_eq!(
            crate::slug("1000 reasons you are #1"),
            "1000-reasons-you-are-1"
        );
    }

    #[pg_test]
    fn test_separator() {
        assert_eq!(crate::slug_sep("hello world", "."), "hello.world");

        assert_eq!(crate::slug_sep("hello world", "_"), "hello_world");
        assert_eq!(
            crate::slug_rand_sep("hello world-", "_").len(),
            "hello_world".len() + 5
        );
    }

    #[pg_test]
    fn test_phonetic_conversion() {
        assert_eq!(crate::slug("影師嗎"), "ying-shi-ma");
    }

    #[pg_test]
    fn test_accented_text() {
        assert_eq!(crate::slug("Æúű--cool?"), "aeuu-cool");
        assert_eq!(
            crate::slug("Nín hǎo. Wǒ shì zhōng guó rén"),
            "nin-hao-wo-shi-zhong-guo-ren"
        );
    }
    #[pg_test]
    fn test_accented_text_non_word_chars() {
        assert_eq!(crate::slug("jaja---lol-méméméoo--a"), "jaja-lol-mememeoo-a");
    }

    #[pg_test]
    fn test_cyrillic_text() {
        assert_eq!(crate::slug("Компьютер"), "komp-iuter");
    }

    #[pg_test]
    fn test_macro() {
        assert_eq!(crate::slug("Компьютер"), "komp-iuter");
        assert_eq!(crate::slug_sep("hello world", "-"), "hello-world");
        assert_eq!(crate::slug_sep("hello world", " "), "hello world");
    }
    #[pg_test]
    fn test_random_length() {
        assert_eq!(
            "hello-world".len() + 5,
            crate::slug_rand("hello world").len()
        );
        assert_eq!(
            "hello_world".len() + 5,
            crate::slug_rand_sep("hello world", "_").len()
        );
    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
