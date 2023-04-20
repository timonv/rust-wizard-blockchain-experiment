use ring::digest;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

const MAGIC_WAND: &[u8] = b"AbraKadabra";

// Define the Wizard struct with name and public_key fields
// TODO use public_key to do some basic encryption for fun
#[derive(Clone, Hash)]
struct Wizard {
    name: String,
    public_key: Vec<u8>,
}

// Define the Spell struct, which represents a block in the blockchain
#[derive(Clone, Hash)]
struct Spell {
    wizard: Wizard,
    magic_word: String,
    timestamp: u64,
    nonce: u64,
    prev_spell_hash: Vec<u8>,
}

// Define the MagicBook struct, which represents the blockchain
struct MagicBook {
    spells: Vec<Spell>,
    wizard_spellbook: HashMap<String, Vec<Spell>>,
}

impl MagicBook {
    // Initialize a new MagicBook with a genesis spell
    fn new() -> Self {
        let genesis_spell = Spell {
            wizard: Wizard {
                name: "Merlin".to_string(),
                public_key: MAGIC_WAND.to_vec(),
            },
            magic_word: "Let the magic begin!".to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            nonce: 0,
            prev_spell_hash: Vec::new(),
        };
        let mut book = MagicBook {
            spells: Vec::new(),
            wizard_spellbook: HashMap::new(),
        };
        book.add_spell(genesis_spell);
        book
    }

    // Add a spell to the MagicBook
    fn add_spell(&mut self, spell: Spell) {
        self.spells.push(spell.clone());
        let entry = self
            .wizard_spellbook
            .entry(spell.wizard.name.clone())
            .or_insert_with(Vec::new);
        entry.push(spell);
    }

    // Validate the new spell by checking the previous spell hash
    fn validate_spell(&self, new_spell: &Spell) -> bool {
        if new_spell.prev_spell_hash != self.compute_hash(&self.spells.last().unwrap()) {
            println!("Invalid spell: previous spell hash mismatch.");
            return false;
        }
        true
    }

    // Compute the hash of an object using the SHA256 algorithm
    fn compute_hash<T: Hash>(&self, t: &T) -> Vec<u8> {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        let hash = s.finish();
        let hash_bytes = hash.to_be_bytes();
        digest::digest(&digest::SHA256, &hash_bytes)
            .as_ref()
            .to_vec()
    }

    // Mine a new spell by finding a nonce that satisfies the difficulty level
    fn mine_spell(&mut self, spell: Spell, difficulty: usize) {
        let mut mined_spell = spell;
        while !self.validate_spell(&mined_spell)
            || !self.hash_matches_difficulty(&mined_spell, difficulty)
        {
            mined_spell.nonce += 1;
        }
        println!("Spell mined: {:?}", mined_spell.magic_word);
        self.add_spell(mined_spell);
    }

    // Check if the hash of a spell matches the difficulty level
    fn hash_matches_difficulty(&self, spell: &Spell, difficulty: usize) -> bool {
        let hash = self.compute_hash(spell);
        hash.iter().take(difficulty).all(|&byte| byte == 0)
    }
}

fn main() {
    let mut magic_book = MagicBook::new();
    let gandalf = Wizard {
        name: "Gandalf".to_string(),
        public_key: vec![1, 2, 3, 4],
    };
    let gandalf_spell = Spell {
        wizard: gandalf,
        magic_word: "You shall not pass!".to_string(),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs(),
        nonce: 0,
        prev_spell_hash: magic_book.compute_hash(magic_book.spells.last().unwrap()),
    };
    // Gandalf mines his spell with a difficulty level of 2
    magic_book.mine_spell(gandalf_spell, 2);
    let dumbledore = Wizard {
        name: "Dumbledore".to_string(),
        public_key: vec![5, 6, 7, 8],
    };
    let dumbledore_spell = Spell {
        wizard: dumbledore,
        magic_word: "Expelliarmus!".to_string(),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs(),
        nonce: 0,
        prev_spell_hash: magic_book.compute_hash(magic_book.spells.last().unwrap()),
    };
    // Dumbledore mines his spell with a difficulty level of 2
    magic_book.mine_spell(dumbledore_spell, 2);

    // Print the spells in the MagicBook
    println!("Magic Book spells:");
    for spell in &magic_book.spells {
        println!(
            "Wizard: {}, Magic Word: {}, Nonce: {}",
            spell.wizard.name, spell.magic_word, spell.nonce
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test if the MagicBook initializes with a genesis spell
    #[test]
    fn test_magic_book_initialization() {
        let magic_book = MagicBook::new();
        assert_eq!(magic_book.spells.len(), 1);
        assert_eq!(magic_book.spells[0].magic_word, "Let the magic begin!");
        assert_eq!(magic_book.spells[0].wizard.name, "Merlin");
    }

    // Test if a spell is added to the MagicBook
    #[test]
    fn test_add_spell() {
        let mut magic_book = MagicBook::new();
        let test_wizard = Wizard {
            name: "TestWizard".to_string(),
            public_key: vec![9, 10, 11, 12],
        };
        let test_spell = Spell {
            wizard: test_wizard,
            magic_word: "TestingMagic!".to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            nonce: 0,
            prev_spell_hash: magic_book.compute_hash(magic_book.spells.last().unwrap()),
        };
        magic_book.add_spell(test_spell);
        assert_eq!(magic_book.spells.len(), 2);
        assert_eq!(magic_book.spells[1].magic_word, "TestingMagic!");
        assert_eq!(magic_book.spells[1].wizard.name, "TestWizard");
    }

    // Test if a mined spell is valid and matches the difficulty level
    #[test]
    fn test_mine_spell() {
        let mut magic_book = MagicBook::new();
        let test_wizard = Wizard {
            name: "TestWizard".to_string(),
            public_key: vec![9, 10, 11, 12],
        };
        let test_spell = Spell {
            wizard: test_wizard,
            magic_word: "TestingMagic!".to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            nonce: 0,
            prev_spell_hash: magic_book.compute_hash(magic_book.spells.last().unwrap()),
        };
        let difficulty = 2;
        magic_book.mine_spell(test_spell, difficulty);
        assert_eq!(magic_book.spells.len(), 2);
        assert!(magic_book.hash_matches_difficulty(&magic_book.spells[1], difficulty));
    }
}
