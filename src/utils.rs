use std::{marker::PhantomData, sync::mpsc::channel};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Position<T = usize> {
    pub row: T,
    pub col: T,
}

macro_rules! pos {
    ($row:expr, $col:expr) => {
        Position {
            row: $row,
            col: $col,
        }
    };
}
pub(crate) use pos;

macro_rules! generate_benchmark {
    ($day:ident) => {
        use paste::paste;

        paste! {
            #[cfg(test)]
            mod tests {
                use super::*;
                use test::Bencher;

                #[bench]
                fn [<bench_ $day _part1>](b: &mut Bencher) {
                    let file = std::fs::read_to_string(concat!("src/", stringify!($day), "/input.txt")).unwrap();

                    b.iter(|| SolverImpl::solve_part1(&file));
                }

                #[bench]
                fn [<bench_ $day _part2>](b: &mut Bencher) {
                    let file = std::fs::read_to_string(concat!("src/", stringify!($day), "/input.txt")).unwrap();

                    b.iter(|| SolverImpl::solve_part2(&file));
                }
            }
        }
    };
}
pub(crate) use generate_benchmark;

impl Position {
    // Note that all of these Position helpers assume that the operation is valid.
    // That is, one should not call `up` on a (0,0) position, as (-1,0) is out of bounds.

    pub fn up(&self, n: usize) -> Self {
        Self {
            row: self.row - n,
            col: self.col,
        }
    }

    pub fn right(&self, n: usize) -> Self {
        Self {
            row: self.row,
            col: self.col + n,
        }
    }

    pub fn down(&self, n: usize) -> Self {
        Self {
            row: self.row + n,
            col: self.col,
        }
    }

    pub fn left(&self, n: usize) -> Self {
        Self {
            row: self.row,
            col: self.col - n,
        }
    }

    pub fn surroundings(&self) -> Vec<Self> {
        vec![self.up(1), self.right(1), self.down(1), self.left(1)]
    }

    pub fn go(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => self.up(1),
            Direction::Right => self.right(1),
            Direction::Down => self.down(1),
            Direction::Left => self.left(1),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn sideways(&self) -> bool {
        *self == Self::Right || *self == Self::Left
    }

    pub fn turn_clockwise(&self) -> Direction {
        match *self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    pub fn turn_counter_clockwise(&self) -> Direction {
        match *self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Up,
            '>' => Self::Right,
            'v' => Self::Down,
            '<' => Self::Left,
            _ => unreachable!(),
        }
    }
}

/// Shards `inputs` uniformly, and runs `f` on one shard per thread, based on the available parallelism of the machine.
/// If `f` requires to use elements captured from the context, this can be passed via the generic `capture` argument.
/// Ultimately, this returns an iterator over the output from each shard.
/// Using this helper only makes sense if `f` takes a substantial amount of time to run, otherwise the cost of sharding
/// and spawning threads will outweigh possible runtime gains.
pub fn shard_and_solve_concurrently<Is, I, C, F, O>(
    inputs: Is,
    capture: C,
    f: F,
) -> std::sync::mpsc::IntoIter<O>
where
    Is: IntoIterator<Item = I>,
    I: Send + 'static,
    C: Clone + Send + 'static,
    F: FnOnce(Vec<I>, C) -> O + Clone + Send + 'static,
    O: Send + 'static,
{
    let (tx, rx) = channel();
    let available_parallelism = std::thread::available_parallelism().unwrap().get();
    let mut shards: Vec<_> = (0..available_parallelism).map(|_| Vec::new()).collect();
    for (i, input) in inputs.into_iter().enumerate() {
        shards[i % available_parallelism].push(input);
    }

    for shard in shards {
        let capture = capture.clone();
        let f = f.clone();
        let tx = tx.clone();
        std::thread::spawn(move || {
            tx.send(f(shard, capture)).unwrap();
        });
    }

    rx.into_iter()
}

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
