
// A fuzz-score that can be used to order results.
#[derive(Debug, PartialEq)]
pub struct Score(pub i64);

// Build a fuzz-score based off number of occurance of a substring.
pub fn substrings(needle: &str, haystack: &str) -> Score
{
    let score = haystack.matches(needle).count();
    Score(score as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_substrings() {
        // Zero matches.
        assert_eq!(substrings("foo", "badger"), Score(0));

        // One match.
        assert_eq!(substrings("foo", "foobar"), Score(1));

        // One match, overlapping.
        assert_eq!(substrings("ofo", "ofofo"), Score(1));

        // Two matches.
        assert_eq!(substrings("foo", "foofoo"), Score(2));
    }
}

