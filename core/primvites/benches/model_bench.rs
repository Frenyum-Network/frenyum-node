use primvites::block_header::{BlockHeader, BlockHeaderBuilder};
use primvites::{BlockNumber, BlockHeight, U256};
use utils::{gas::Gas, timestamp::Timestamp};
use crypto::hash::HashDigest;
use rand::{random, Rng, thread_rng};
use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};

fn random_hash() -> HashDigest
{
    let hash = HashDigest::from(rand::random::<[u8; 32]>());
    hash
}

fn bench_block_header_creation(c: &mut Criterion)
{
    let mut builder = BlockHeaderBuilder::new();
    let mut rng = thread_rng();

    c.bench_function("block_header_creation", |b|
    {
        b.iter(|| {
            let hash = random_hash();
            let protocol_version = 1 + rng.gen::<u32>();
            let parent_hash = random_hash();
            let block_number = rng.gen::<BlockNumber>();
            let block_height = rng.gen::<BlockHeight>();
            let difficulty = U256::from(rng.gen::<u64>());
            let timestamp = Timestamp::now();
            let nonce = U256::from(rng.gen::<u64>());
            let total_difficulty = U256::from(rng.gen::<u64>());
            let gas_used = Gas::new(rng.gen::<u64>());
            let gas_limit = Gas::new(rng.gen::<u64>());
            let transaction_root = random_hash();

            let header = builder
                .set_hash(hash.clone())
                .set_protocol_version(protocol_version)
                .set_parent_hash(parent_hash.clone())
                .set_block_number(block_number)
                .set_block_height(block_height)
                .set_difficulty(difficulty.clone())
                .set_timestamp(timestamp.clone())
                .set_nonce(nonce.clone())
                .set_total_difficulty(total_difficulty.clone())
                .set_gas_used(gas_used.clone())
                .set_gas_limit(gas_limit.clone())
                .set_transaction_root(transaction_root.clone())
                .build();
            
            black_box(header);
        })
    });
}

criterion_group!(benches, bench_block_header_creation);
criterion_main!(benches);

