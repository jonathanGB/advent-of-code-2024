use std::sync::mpsc::channel;

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
}

/// Shards `inputs` uniformly, and runs `f` on one shard per thread, based on the available parallelism of the machine.
/// If `f` requires to use elements captured from the context, this can be passed via the generic `capture` argument.
/// Ultimately, this returns an iterator over the output from each shard.
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
