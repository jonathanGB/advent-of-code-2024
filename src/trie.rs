use std::marker::PhantomData;

pub trait TrieElement {
    fn index(&self) -> usize;
}

#[derive(Debug)]
pub struct Trie<T, const N: usize> {
    trie_entries: Vec<TrieEntry<N>>,
    element: PhantomData<T>,
}

impl<T, const N: usize> Trie<T, N>
where
    T: TrieElement,
{
    fn add_word(&mut self, word: impl IntoIterator<Item = T>) {
        let mut last_trie_entry_index = 0;

        for c in word {
            let c_index = c.index();

            if self.trie_entries[last_trie_entry_index].entries[c_index].is_none() {
                self.trie_entries[last_trie_entry_index].entries[c_index] =
                    Some(self.trie_entries.len());
                self.trie_entries.push(TrieEntry::default());
            }

            last_trie_entry_index =
                self.trie_entries[last_trie_entry_index].entries[c_index].unwrap();
        }

        self.trie_entries[last_trie_entry_index].terminal = true;
    }

    pub fn count_all_word_arrangements(&self, word: &[T]) -> u64 {
        // +1 because index 0 is the special index to start with. What this records,
        // using dynamic programming, is that at index N+1, X arrangements reach N.
        // This could be one word from 0 to N, or maybe one word from 0 to K and one from
        // K+1 to N, and so on.
        let mut count_arrangements_reaching_index = vec![0; word.len() + 1];
        count_arrangements_reaching_index[0] = 1;

        // Iterate in-order through prefixes starting at all positions of the word.
        for start_prefix in 0..word.len() {
            // If there are no arrangements terminating at this index, then we can ignore it.
            if count_arrangements_reaching_index[start_prefix] == 0 {
                continue;
            }

            let mut last_trie_entry_index = 0;

            // Iterate through all possible [start_prefix:end_prefix] substrings in the given word,
            // unless we potentially reach the point at which we know no future substrings will exist
            // in the trie.
            for end_prefix in start_prefix..word.len() {
                let c_index = word[end_prefix].index();
                match self.trie_entries[last_trie_entry_index].entries[c_index] {
                    Some(current_trie_entry_index) => {
                        // If there is a word from `start_prefix` that terminates at `end_prefix`,
                        // add up previous arrangements leading up to here.
                        if self.trie_entries[current_trie_entry_index].terminal {
                            count_arrangements_reaching_index[end_prefix + 1] +=
                                count_arrangements_reaching_index[start_prefix];
                        }

                        last_trie_entry_index = current_trie_entry_index;
                    }
                    // There is no word from `start_prefix` that reaches `end_prefix`, stop.
                    None => break,
                }
            }
        }

        count_arrangements_reaching_index[word.len()]
    }
}

impl<T, const N: usize> Default for Trie<T, N> {
    fn default() -> Self {
        Self {
            trie_entries: vec![TrieEntry::default()],
            element: PhantomData,
        }
    }
}

impl<Ts, T, const N: usize> FromIterator<Ts> for Trie<T, N>
where
    Ts: IntoIterator<Item = T>,
    T: TrieElement,
{
    fn from_iter<I: IntoIterator<Item = Ts>>(iter: I) -> Self {
        let mut trie = Trie::default();

        for word in iter {
            trie.add_word(word);
        }

        trie
    }
}

#[derive(Debug)]
struct TrieEntry<const N: usize> {
    entries: [Option<usize>; N],
    terminal: bool,
}

impl<const N: usize> Default for TrieEntry<N> {
    fn default() -> Self {
        Self {
            entries: [None; N],
            terminal: false,
        }
    }
}
