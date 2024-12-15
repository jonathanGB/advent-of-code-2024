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
