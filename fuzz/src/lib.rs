use std::cmp::Ordering;
use std::path::PathBuf;

// A fuzz-score that can be used to order results.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Score(pub i64);

// A found path coupled with it's score.
#[derive(Debug, Eq, PartialEq)]
pub struct Matched {
    pub score: Score,
    pub path: PathBuf,
}

impl Ord for Matched {
    fn cmp(&self, other: &Matched) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for Matched {
    fn partial_cmp(&self, other: &Matched) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Build a fuzz-score based off number of occurance of a substring.
pub fn substrings(needle: &str, haystack: &str) -> Score
{
    if needle.len() == 0 {
        Score(0)
    } else {
        let score = haystack.matches(needle).count();
        Score(score as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_substrings() {
        // Zero matches.
        assert_eq!(substrings("", "badger"), Score(0));
        assert_eq!(substrings("foo", "badger"), Score(0));

        // One match.
        assert_eq!(substrings("foo", "foobar"), Score(1));

        // One match, overlapping.
        assert_eq!(substrings("ofo", "ofofo"), Score(1));

        // Two matches.
        assert_eq!(substrings("foo", "foofoo"), Score(2));
    }
}

