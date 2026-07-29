use crate::{cli::Args, model::Config};

pub fn from_args(args: Args) -> Config {
    Config {
        root: args.path,
        exclude: args.exclude,
        follow_symlinks: false,
        print_tree: !args.no_tree,
        output: args.output,
    }
}
