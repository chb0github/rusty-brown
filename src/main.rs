use std::io::{BufRead, BufReader, stdin};
use crate::block_chain::BlockChain;


mod block_chain;


fn main() {
    let mut bc = BlockChain::new();
    bc.genesis();
    println!("Please one block data entry per line.");
    println!("'quit' to exit");
    println!("'dump' to dump the entire block chain");

    BufReader::new(stdin()).lines()
        .map(|l| l.unwrap())
        .take_while(|l| l != "quit").for_each(|l|
            match l.as_str() {
                "dump" => bc.dump(),
                _ => {
                    bc.mk_block( l.clone()).map(|block| bc.add_block(block)).expect("couldn't add block");

                }
            }
        )
}

