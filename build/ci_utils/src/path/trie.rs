use crate::prelude::*;



#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Trie<'a> {
    pub children: HashMap<std::path::Component<'a>, Trie<'a>>,
    pub count:    usize,
}

impl<'a> Trie<'a> {
    pub fn insert(&mut self, path: &'a Path) {
        let mut current = self;
        for component in path.components() {
            current = current.children.entry(component).or_default();
        }
        current.count += 1;
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

impl<'a> FromIterator<&'a Path> for Trie<'a> {
    fn from_iter<I: IntoIterator<Item = &'a Path>>(iter: I) -> Self {
        let mut trie = Trie::default();
        trie.extend(iter);
        trie
    }
}

impl<'a> FromIterator<&'a PathBuf> for Trie<'a> {
    fn from_iter<I: IntoIterator<Item = &'a PathBuf>>(iter: I) -> Self {
        let mut trie = Trie::default();
        trie.extend(iter);
        trie
    }
}

impl<'a> Extend<&'a Path> for Trie<'a> {
    fn extend<I: IntoIterator<Item = &'a Path>>(&mut self, iter: I) {
        for path in iter {
            self.insert(path);
        }
    }
}

impl<'a> Extend<&'a PathBuf> for Trie<'a> {
    fn extend<I: IntoIterator<Item = &'a PathBuf>>(&mut self, iter: I) {
        for path in iter {
            self.insert(path);
        }
    }
}
