use std::io::{BufRead, BufReader, stdin};
use crate::block_chain::BlockChain;


mod block_chain;


fn main() {
    let mut bc = BlockChain::new();

    BufReader::new(stdin()).lines()
        .map(|l| l.unwrap())
        .take_while(|l| l != "quit").for_each(|l| { bc.add_block(l); })
}
