use clap::{Parser, Subcommand};
use seq_macro::seq;

#[derive(Parser, Debug)]
#[command(author, version)]
pub struct Args {
    #[command(subcommand)]
    pub day: Day,
}

seq!(N in 1..=25 {
    #[derive(Subcommand, Debug)]
    pub enum Day {
        #(
            #[command(about = format!("Solve day {}'s problem.", N))]
            Day~N {
                #[command(subcommand)]
                part: Part,

                #[arg(short, long, default_value = "input", global=true)]
                /// Name of the input file. Do not include the path nor the file extension (must be `.txt`).
                input: String,
            },
        )*
    }
});

#[derive(Subcommand, Debug)]
pub enum Part {
    /// Solve the 1st part of the problem.
    Part1,
    /// Solve the 2nd part of the problem.
    Part2,
}
