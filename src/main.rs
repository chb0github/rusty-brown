use std::io::{BufRead, BufReader, stdin};
use std::process::exit;
use crate::block_chain::BlockChain;


mod block_chain;


fn main() {
    let mut bc = BlockChain::new();
    bc.genesis();

    BufReader::new(stdin()).lines()
        .map(|l| l.unwrap())
        .take_while(|l| l != "quit").for_each(|l|
            match l.as_str() {
                "quit" => exit(0),
                "dump" => bc.dump(),
                _ => { bc.add_block(l); }
            }
        )
}

