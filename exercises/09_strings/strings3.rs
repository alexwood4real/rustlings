fn trim_me(input: &str) -> &str {
    // TODO: Remove whitespace from both ends of a string.
    let trime_ws: &[char] = &[ ' ' ];
    let trimmed_str: &str = input.trim_matches( trime_ws );
    trimmed_str
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this.
    let mut world_str: String  = String::new();
    world_str.push_str( input );
    world_str.push_str( " world!");
    world_str
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons".
    let replaced: String = input.replace( "cars", "balloons" );
    replaced
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
        assert_eq!(trim_me("Hi!"), "Hi!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}
