# poke_search_cli

# Installation
You will need Rust 1.66.0 installed. I recommend using [`rustup`](https://rustup.rs/)
```sh
# From the rustup.rs website
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

And then run
```sh
rustup default 1.66.0
```

Then

1. Clone the repository
2. Run `cargo build --release`

You now have a release build! This can be run with `./target/release/poke_search_cli` assuming you're in the root directory of this repository

This project doesn't currently distrbute or release any binaries. For now I'd recommend using an alias if you intend on using this outside of the repo directly
```sh
alias poke_search_cli="/link/to/poke_search_cli/target/release/poke_search_cli"
```

# Usage
See the `help` command for a list of commands that can be run
```sh
❯ poke_search_cli help
Search for pokemon information from the command line

Usage: poke_search_cli <COMMAND>

Commands:
  moves    See moves for a pokemon
  move     See information about a move
  pokemon  See information about a pokemon
  type     See information about a specific type
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help information
```

### Example
![image](https://user-images.githubusercontent.com/13454550/209565900-6de7db16-6f80-4fba-853b-d130cd05c066.png)
