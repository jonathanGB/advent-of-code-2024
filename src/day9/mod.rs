use std::{cmp::Reverse, collections::BinaryHeap};

use crate::{solver::Solver, utils::generate_benchmark};

macro_rules! offset_based_ord_and_eq {
    ($T:ident) => {
        impl Ord for $T {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.pos_offset.cmp(&other.pos_offset)
            }
        }

        impl PartialOrd for $T {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl PartialEq for $T {
            fn eq(&self, other: &Self) -> bool {
                self.pos_offset == other.pos_offset
            }
        }

        impl Eq for $T {}
    };
}

#[derive(Debug)]
struct FileBlock {
    id: usize,
    pos_offset: usize,
    num_blocks: usize,
}

offset_based_ord_and_eq!(FileBlock);

#[derive(Debug)]
struct FreeBlock {
    pos_offset: usize,
    moved_file_blocks: Vec<FileBlock>,
}

offset_based_ord_and_eq!(FreeBlock);

#[derive(Debug)]
struct Compaction {
    file_blocks: Vec<FileBlock>,
}

impl Compaction {
    fn new(disk_map: &str) -> Self {
        assert!(disk_map.len() % 2 == 1);

        let mut file_blocks = Vec::new();
        let mut space_layout = disk_map.chars();
        let mut pos_offset = 0;
        let mut front_id = 0;
        let mut back_id = (disk_map.len() - 1) / 2;
        let mut num_back_blocks_to_move =
            space_layout.next_back().unwrap().to_digit(10).unwrap() as usize;

        'compactions: loop {
            let mut num_free_blocks = match space_layout.next() {
                Some(num_front_blocks) => {
                    // Move forward, and append this file block.
                    let num_front_blocks = num_front_blocks.to_digit(10).unwrap() as usize;
                    file_blocks.push(FileBlock {
                        id: front_id,
                        pos_offset,
                        num_blocks: num_front_blocks,
                    });
                    pos_offset += num_front_blocks;
                    front_id += 1;

                    space_layout
                        .next()
                        .expect(
                            "It should be impossible to fail getting the next number of free blocks if we were able to get the previous number of front file blocks",
                        )
                        .to_digit(10)
                        .unwrap() as usize
                }
                // If we couldn't advance forward, then we may still be trying to compact
                // a file block from the back. Make it simple and tell the compaction
                // that there is effectively no maximum of free blocks left.
                None => usize::MAX,
            };

            // Try to compact files from the back.
            while num_back_blocks_to_move > 0 && num_free_blocks > 0 {
                let num_back_blocks_moved = num_back_blocks_to_move.min(num_free_blocks);
                if num_back_blocks_moved > 0 {
                    file_blocks.push(FileBlock {
                        id: back_id,
                        pos_offset,
                        num_blocks: num_back_blocks_moved,
                    });
                    pos_offset += num_back_blocks_moved;
                    num_back_blocks_to_move -= num_back_blocks_moved;
                    num_free_blocks -= num_back_blocks_moved;
                }

                // There is still more to compact from this file at the back, but there is evidently
                // no free spaces left at the front. Stop for now.
                if num_back_blocks_to_move > 0 {
                    continue 'compactions;
                }

                // We succeeded in compacting a full file from the back.
                // We can move on to the next file from the back to compact.
                // One benefit is that we can skip the next entry from the back,
                // which is a redundant FreeBlock. We say redundant because necessarily
                // all files will be compacted using this scheme before the skipped FreeBlock.
                space_layout.next_back();

                // Try another file to compact. If there is none, we are done!
                match space_layout.next_back() {
                    Some(num_back_blocks) => {
                        num_back_blocks_to_move = num_back_blocks.to_digit(10).unwrap() as usize;
                        back_id -= 1;
                    }
                    None => break 'compactions,
                }
            }
        }

        Self { file_blocks }
    }

    fn new_without_fragmentation(disk_map: &str) -> Self {
        // Array of length 10.
        // Index 0 stores all the FreeBlocks with 0 unused blocks;
        // Index 1 stores all the FreeBlocks with 1 unused blocks;
        // And so on...
        // Due to the single digit representation, we know that FreeBlocks and
        // FileBlocks are at most size 9.
        // Each index stores a min-heap of FreeBlocks, so that we can peek in O(1) time
        // at the left-most FreeBlock with N unused blocks.
        let mut free_blocks_by_unused_size: [BinaryHeap<Reverse<FreeBlock>>; 10] =
            Default::default();
        let mut file_blocks = Vec::new();
        let mut pos_offset = 0;

        for (i, num_blocks) in disk_map.char_indices() {
            let num_blocks = num_blocks.to_digit(10).unwrap() as usize;

            if i % 2 == 0 {
                // Efficient division by 2, as we know `i` is a multiple of 2.
                // Ids only increment each FileBlock, which is every 2 iterations.
                let id = i >> 1;

                file_blocks.push(FileBlock {
                    id,
                    pos_offset,
                    num_blocks,
                });
            } else {
                // Optimization: discard initial empty FreeBlocks. They will never be useful.
                if num_blocks == 0 {
                    continue;
                }

                let free_block = FreeBlock {
                    pos_offset,
                    moved_file_blocks: Vec::new(),
                };
                free_blocks_by_unused_size[num_blocks].push(Reverse(free_block));
            }

            pos_offset += num_blocks;
        }

        // Now, figure out, right to left, which FileBlocks are moved during compaction. If so, they should be moved
        // inside the left-most FreeBlock that can fit them. The latter will itself need to be moved
        // to reflect the number of blocks left in it.
        let mut unmoved_file_blocks = Vec::new();
        for mut file_block in file_blocks.into_iter().rev() {
            let num_file_blocks = file_block.num_blocks;

            match free_blocks_by_unused_size[num_file_blocks..]
                .iter()
                .enumerate()
                .map(|(free_blocks_index_offset, free_blocks)| {
                    // The `free_blocks_index_offset` is propagated so we can keep track of which index to access
                    // once we've found the left-most FreeBlock, i.e. the one with the smallest position offset.
                    // Note that the tuple is sorted lexicographically, and because all offsets are unique,
                    // the 2nd entry in the tuple is never used to find the minimum.
                    Some((free_blocks.peek()?.0.pos_offset, free_blocks_index_offset))
                })
                // Ignore empty min-heaps.
                .flatten()
                .min()
            {
                // Case where we found a FreeBlock with enough space and which is to the left of the FileBlock.
                // The previous search further guarantees that this FreeBlock is the left-most one.
                Some((pos_offset, free_blocks_index_offset))
                    if pos_offset < file_block.pos_offset =>
                {
                    let free_blocks_index = num_file_blocks + free_blocks_index_offset;
                    let mut free_block = free_blocks_by_unused_size[free_blocks_index]
                        .pop()
                        .unwrap()
                        .0;
                    let unused_blocks = free_blocks_index - file_block.num_blocks;
                    file_block.pos_offset = match free_block.moved_file_blocks.last() {
                        Some(last_embedded_file_block) => {
                            last_embedded_file_block.pos_offset
                                + last_embedded_file_block.num_blocks
                        }
                        None => free_block.pos_offset,
                    };
                    free_block.moved_file_blocks.push(file_block);
                    free_blocks_by_unused_size[unused_blocks].push(Reverse(free_block));
                }
                // Otherwise, the FileBlock is unmoved.
                _ => unmoved_file_blocks.push(file_block),
            }
        }

        // Re-assemble all of the FileBlocks, sorted by position offset.
        let mut file_blocks = unmoved_file_blocks;
        file_blocks.extend(
            free_blocks_by_unused_size
                .into_iter()
                .flatten()
                .map(|free_block| free_block.0.moved_file_blocks)
                .flatten(),
        );
        file_blocks.sort();

        Self { file_blocks }
    }

    fn check_sum(&self) -> usize {
        self.file_blocks
            .iter()
            .map(|file_block| {
                (file_block.pos_offset..file_block.pos_offset + file_block.num_blocks)
                    .map(|i| i * file_block.id)
                    .sum::<usize>()
            })
            .sum()
    }
}

pub struct SolverImpl {}

impl Solver for SolverImpl {
    fn solve_part1(file: &str) {
        let compaction = Compaction::new(file);
        println!("The checksum is {}", compaction.check_sum());
    }

    fn solve_part2(file: &str) {
        let compaction = Compaction::new_without_fragmentation(file);
        println!("The checksum is {}", compaction.check_sum());
    }
}

generate_benchmark!(day9);
