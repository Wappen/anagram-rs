use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::{fs, io};

pub trait Anagrams {
    fn get_all(&self, word: &str) -> Option<&Vec<String>>;
}

struct AnagramID {
    count_map: BTreeMap<char, usize>,
}

impl PartialEq<Self> for AnagramID {
    fn eq(&self, other: &Self) -> bool {
        return self.count_map.len() == other.count_map.len()
            && self.count_map.keys().eq(other.count_map.keys())
            && self.count_map.values().eq(other.count_map.values());
    }
}

impl Eq for AnagramID {}

impl Hash for AnagramID {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.count_map.len().hash(state);

        for char_count in self.count_map.iter() {
            char_count.hash(state);
        }
    }
}

impl From<&str> for AnagramID {
    fn from(str: &str) -> Self {
        let mut count_map: BTreeMap<char, usize> = BTreeMap::new();

        for c in str.chars() {
            count_map.insert(c, *count_map.get(&c).unwrap_or(&0) + 1);
        }

        AnagramID { count_map }
    }
}

pub struct AnagramWordList {
    word_map: HashMap<AnagramID, Vec<String>>,
}

impl AnagramWordList {
    pub fn load_from_file(
        path: &Path,
        word_preprocess: fn(&str) -> String,
    ) -> io::Result<AnagramWordList> {
        let mut word_map: HashMap<AnagramID, Vec<String>> = HashMap::new();

        let content = fs::read_to_string(path)?;
        let lines = content.split('\n');

        for line in lines {
            let word = word_preprocess(line);
            let id = AnagramID::from(word.as_str());
            let vec = word_map.get_mut(&id);

            if let Some(vec) = vec {
                vec.push(word.to_string());
            } else {
                let vec = vec![word.to_string()];
                word_map.insert(id, vec);
            }
        }

        Ok(AnagramWordList { word_map })
    }
}

impl Anagrams for AnagramWordList {
    fn get_all(&self, word: &str) -> Option<&Vec<String>> {
        let id = AnagramID::from(word);
        self.word_map.get(&id)
    }
}
