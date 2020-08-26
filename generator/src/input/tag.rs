use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/// important: We want do identify tags by their normalized names (instead of their display names).
/// For that, Eq and Ord are implemented manually.
/// I'm not really sure if this is a good idea from an engineering standpoint, but hell, I am and
/// I will most likely be the only maintainer for this software, so I don't care.
#[derive(Clone, Eq, Debug)]
pub struct Tag {
    pub name: String,
    pub normalized_name: String,
}

impl PartialEq for Tag {
    fn eq(&self, other: &Tag) -> bool {
        self.normalized_name == other.normalized_name
    }
}

impl PartialOrd for Tag {
    fn partial_cmp(&self, other: &Tag) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Tag {
    fn cmp(&self, other: &Tag) -> Ordering {
        self.normalized_name.cmp(&other.normalized_name)
    }
}

impl Hash for Tag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.normalized_name.hash(state);
    }
}

impl Tag {
    pub fn new(name: &str) -> Tag {
        let lowercase = name.to_lowercase();
        // We _could_ use a cow-str here, but this whole stuff is fast enough as it is.
        let normalized_name: String = lowercase
            .to_lowercase()
            .trim()
            .chars()
            .filter(|c| *c != '\'' && *c != '"')
            .map(|c| if c == ' ' { '-' } else { c })
            .collect();
        Tag {
            name: lowercase,
            normalized_name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;

    #[test]
    fn tag_new_normalizes_tags_correctly() {
        // given
        let raw_name = "Foo's \"bar\"";

        // when
        let tag = Tag::new(raw_name);

        // then
        assert_eq!(tag.name, "foo's \"bar\"");
        assert_eq!(tag.normalized_name, "foos-bar");
    }

    fn hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    #[test]
    fn tag_hash_is_based_only_on_normalized_name() {
        // given
        // two Tags with different names that are normalized to the same name
        let tag1 = Tag::new("Foo's bar");
        let tag2 = Tag::new("foos-bar");

        // when
        let tag1_hash = hash(&tag1);
        let tag2_hash = hash(&tag2);
        let unnormalized_hash = hash(&tag1.name);

        // then
        assert_eq!(tag1_hash, tag2_hash);
        assert_ne!(tag1_hash, unnormalized_hash);
    }

    #[test]
    fn tag_eq_is_based_only_on_normalized_name() {
        // given
        // two Tags with different names that are normalized to the same name
        let tag1 = Tag::new("Foo's bar");
        let tag2 = Tag::new("foos-bar");
        // a different tag
        let tag3 = Tag::new("Something else");

        // when / then
        assert_eq!(tag1, tag2);
        assert_ne!(tag1, tag3);
    }

    #[test]
    fn tag_cmp_is_based_only_on_normalized_name() {
        // given
        // two Tags with different names that are normalized to the same name
        let tag1 = Tag::new("Foo's bar");
        let tag2 = Tag::new("foos-bar");
        // a different tag
        let tag3 = Tag::new("Something else");

        // when / then
        assert_eq!(tag1.cmp(&tag2), Ordering::Equal);
        assert_eq!(tag1.cmp(&tag3), Ordering::Less);
    }
}
