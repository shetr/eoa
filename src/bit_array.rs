use std::mem;

pub struct BitArray<const BLOCK_COUNT: usize>
{
    bit_count: usize,
    blocks: [u64; BLOCK_COUNT]
}

pub struct BitVec
{
    bit_count: usize,
    blocks: Vec<u64>
}

fn get_bit(blocks: &[u64], bit_count: usize, bit_index: usize) -> bool {
    if bit_index >= bit_count {
        panic!("bit index out of range");
    }
    let in_block_offset = bit_index % mem::size_of::<u64>();
    let block = blocks[bit_index / mem::size_of::<u64>()];
    (block & (1 << in_block_offset)) > 0
}

fn set_bit(blocks: &mut [u64], bit_count: usize, bit_index: usize, value: bool) {
    if bit_index >= bit_count {
        panic!("bit index out of range");
    }
    let in_block_offset = bit_index % mem::size_of::<u64>();
    let block = &mut blocks[bit_index / mem::size_of::<u64>()];
    let shifted_value = (value as u64) << in_block_offset;
    *block = (*block & (!(1 << in_block_offset))) | shifted_value;
}

pub fn perturbe_bits_naive(blocs: &mut [u64], bit_count: usize, prob: f64) {
    for bit_index in 0..bit_count {
        let old_value = get_bit(blocs, bit_count, bit_index);
        let new_value = if rand::random::<f64>() > prob { old_value } else { old_value ^ true };
        set_bit(blocs, bit_count, bit_index, new_value);
    }
}

pub fn perturbe_bits(blocs: &mut [u64], one_over_prob_pow_of_2: usize) {
    for block in blocs {
        let mut perturbe_bits = u64::MAX;
        for _ in 0..one_over_prob_pow_of_2 {
            perturbe_bits &= rand::random::<u64>();
        }
        *block = *block ^ perturbe_bits;
    }
}

impl<const BLOCK_COUNT: usize> BitArray<{BLOCK_COUNT}> {

    pub fn zero(bit_count: usize) -> Self {
        let blocks: [u64; BLOCK_COUNT] = [0; BLOCK_COUNT];
        BitArray { bit_count, blocks }
    }

}

impl BitVec {
    
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn zero() {
        let zero = BitArray::<1>::zero(1);
    }
}