use blockchain::BlockChain;
use bytes::Bytes;

fn main() {
    let mut chain = BlockChain::new();
    chain.add_block(Bytes::from("Hello, World"));

    for block in chain.blocks {
        println!("{}", block.details());
    }
}
