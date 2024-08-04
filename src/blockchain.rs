use crate::block::Block;

type Blocks = Vec<Block>;
// `Blockchain` A struct that represents the blockchain.
#[derive(Debug)]
pub struct Blockchain {
    // The first block to be added to the chain.
    pub genesis_block: Block,
    // The storage for blocks.
    pub chain: Blocks,
    // Minimum amount of work required to validate a block.
    pub difficulty: usize
}
impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        // first block in the chain
        let genesis_block = Block::generate_genesis_block();
        // Create a chain starting from the genesis chain.
        let mut chain = Vec::new();
        chain.push(genesis_block.clone());

        Blockchain{genesis_block, chain, difficulty}
    }

    pub fn add_block(&mut self, nonce: u64) {
        let mut new_block = Block::new(self.chain.len() as u64, nonce, self.chain[&self.chain.len() - 1].hash.clone());

        self.mine_block(&mut new_block);

        self.chain.push(new_block.clone());
        println!("New block added to chain -> {:?}", new_block)
    }

    pub fn mine_block(&self, block: &mut Block) {
        loop {
            if !block.hash.starts_with(&"0".repeat(self.difficulty)) {
                block.proof_of_work += 1;
                block.hash = Block::calculate_hash(&block);
            } else {
                break
            }
        }
    }
}