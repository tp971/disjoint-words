use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone)]
pub struct Tree {
    chars: Vec<char>,
    words: Vec<String>,
    childs: BTreeMap<char, Tree>,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            chars: Vec::new(),
            words: Vec::new(),
            childs: BTreeMap::new(),
        }
    }

    fn new_child(parent: &Tree, ch: char) -> Self {
        Self {
            chars: [&parent.chars[..], &[ch]].concat(),
            words: Vec::new(),
            childs: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut chars = word.chars()
            .collect::<Vec<_>>();
        chars.sort();
        self.insert_impl(&chars, word);
    }

    fn insert_impl(&mut self, chars: &[char], word: String) {
        if chars.is_empty() {
            self.words.push(word);
            return;
        }
        if !self.childs.contains_key(&chars[0]) {
            let child = Tree::new_child(self, chars[0]);
            self.childs.insert(chars[0], child);
        }
        self.childs.get_mut(&chars[0]).unwrap()
            .insert_impl(&chars[1..], word);
    }

    pub fn all_words(&self) -> Vec<&String> {
        let mut out = Vec::new();
        self.all_words_impl(&mut out);
        out
    }

    fn all_words_impl<'a>(&'a self, out: &mut Vec<&'a String>) {
        out.extend(self.words.iter());
        for child in self.childs.values() {
            child.all_words_impl(out);
        }
    }

    pub fn build_fast_tree(&self) -> FastTree {
        let words = self.all_words();
        let chars = words.iter()
            .flat_map(|w| w.chars());

        let mut map = HashMap::new();
        for ch in chars {
            *map.entry(ch).or_insert(0usize) += 1;
        }

        self.build_fast_tree_impl(&map)
    }

    fn build_fast_tree_impl(&self, map: &HashMap<char, usize>) -> FastTree {
        let mut keys = self.childs.keys()
            .copied()
            .collect::<Vec<char>>();
        keys.sort_by_key(|c| (map[c], *c));

        FastTree {
            chars: self.chars.clone(),
            words: self.words.clone(),
            childs: keys.into_iter()
                .map(|ch| (ch, self.childs[&ch].build_fast_tree()))
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FastTree {
    chars: Vec<char>,
    words: Vec<String>,
    childs: Vec<(char, FastTree)>,
}

impl FastTree {
    pub fn chars(&self) -> &[char] {
        &self.chars
    }

    pub fn words(&self) -> &[String] {
        &self.words
    }

    pub fn childs(&self) -> impl Iterator<Item = (char, &FastTree)> {
        self.childs.iter()
            .map(|(k, v)| (*k, v))
    }

    pub fn get_all_word_nodes(&self) -> Vec<&FastTree> {
        let mut out = Vec::new();
        self.get_all_word_nodes_impl(&mut out);
        out
    }

    pub fn get_all_word_nodes_impl<'a>(&'a self, out: &mut Vec<&'a FastTree>) {
        if !self.words().is_empty() {
            out.push(self);
        }
        for (_, next) in &self.childs {
            next.get_all_word_nodes_impl(out);
        }
    }
}
