pub mod block;
// mod proof_of_work;
// mod Transaction;
pub mod blockchain;

fn main() {
   let difficulty = 1;
   let mut blockchain = blockchain::Blockchain::new(difficulty);
    blockchain.add_block(10);
    blockchain.add_block(10);
    blockchain.add_block(10);
    blockchain.add_block(10);
    blockchain.add_block(10);
    blockchain.add_block(10);
    blockchain.add_block(10);
    blockchain.add_block(10);
    blockchain.add_block(10);
    blockchain.add_block(10);
}
