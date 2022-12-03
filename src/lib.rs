use std::collections::{BTreeMap, HashMap};
use std::path::Path;
use std::{fs, io};

pub trait Anagrams {
    fn get_all(&self, word: String) -> Option<&Vec<String>>;
}

#[derive(Eq, PartialEq, Hash)]
struct AnagramID {
    count_map: BTreeMap<char, usize>,
}

impl From<&String> for AnagramID {
    fn from(str: &String) -> Self {
        let mut count_map: BTreeMap<char, usize> = BTreeMap::new();

        for c in str.chars() {
            count_map.insert(c, *count_map.get(&c).unwrap_or(&1));
        }

        AnagramID { count_map }
    }
}

pub struct AnagramWordList {
    word_map: HashMap<AnagramID, Vec<String>>,
}

impl AnagramWordList {
    pub fn load_from_file(path: &Path) -> io::Result<AnagramWordList> {
        let mut word_map: HashMap<AnagramID, Vec<String>> = HashMap::new();

        let content = fs::read_to_string(path)?;
        let lines = content.split('\n');

        for line in lines {
            let word = line.trim().to_string();
            let id = AnagramID::from(&word);
            let vec = word_map.get_mut(&id);

            if let Some(vec) = vec {
                vec.push(word);
            } else {
                let vec = vec![word];
                word_map.insert(id, vec);
            }
        }

        Ok(AnagramWordList { word_map })
    }
}

impl Anagrams for AnagramWordList {
    fn get_all(&self, word: String) -> Option<&Vec<String>> {
        let id = AnagramID::from(&word);
        self.word_map.get(&id)
    }
}
