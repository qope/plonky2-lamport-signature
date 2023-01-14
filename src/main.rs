use std::time::Instant;

use plonky2::{
    field::{
        goldilocks_field::GoldilocksField,
        types::{Field, PrimeField64, Sample},
    },
    hash::{
        hash_types::{HashOut, HashOutTarget, RichField},
        merkle_tree::MerkleTree,
        poseidon::PoseidonHash,
    },
    iop::{
        target::{Target, BoolTarget},
        witness::{PartialWitness, WitnessWrite},
    },
    plonk::{
        circuit_builder::CircuitBuilder,
        circuit_data::CircuitConfig,
        config::{GenericConfig, Hasher, PoseidonGoldilocksConfig},
    },
};

const D: usize = 2;
type C = PoseidonGoldilocksConfig;
type F = GoldilocksField;
type H = PoseidonHash;

fn main() {
    make_signature();
    let config = CircuitConfig::standard_recursion_config();
    let n = 256;
    let preimages: Vec<Vec<F>> = (0..n).map(|_| F::rand_vec(4)).collect();
    let images: Vec<HashOut<F>> = preimages.iter().map(|x| H::hash_no_pad(x)).collect();
    let mut builder = CircuitBuilder::<F, D>::new(config.clone());
    let mut pw = PartialWitness::<F>::new();
    for i in 0..n {
        let targets = builder.add_virtual_targets(4);
        for j in 0..4 {
            pw.set_target(targets[j], preimages[i][j]);
        }
        let hash_target = builder.hash_n_to_hash_no_pad::<H>(targets);
        pw.set_hash_target(hash_target, images[i]);
    builder.select(b, x, y)
    let data = builder.build::<C>();
    let now = Instant::now();
    let _proof = data.prove(pw).unwrap();
    dbg!(data.common.degree_bits());
    dbg!(now.elapsed().as_millis());
    // println!("Hello, world!");
}

fn make_signature() {
    let config = CircuitConfig::standard_recursion_config();
    let n = 256;
    let preimages: Vec<Vec<F>> = (0..2 * n).map(|_| F::rand_vec(4)).collect();
    let images: Vec<HashOut<F>> = preimages.iter().map(|x| H::hash_no_pad(x)).collect();
    let tree = MerkleTree::<F, H>::new(images.iter().map(|x| x.elements.to_vec()).collect(), 0);
    let root: HashOut<F> = tree.cap.0[0];
    let msg = H::hash_pad(&vec![F::ONE]).elements.to_vec();
    let v: Vec<bool> = msg
        .iter()
        .map(|x| u64_to_vec(x.to_canonical_u64()))
        .flatten()
        .collect();
    let mut selected_preimage = vec![];
    let mut selected_image = vec![];
    for i in 0..n {
        selected_preimage.push(if v[i] {
            preimages[2 * i + 1].clone()
        } else {
            preimages[2 * i].clone()
        });
        selected_image.push(if v[i] {
            images[2 * i + 1].clone()
        } else {
            images[2 * i].clone()
        });
    }
    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, D>::new(config.clone());
    let mut pw = PartialWitness::<F>::new();
    let images_t: Vec<HashOutTarget> = (0..2 * n).map(|_| builder.add_virtual_hash()).collect();
    for i in 0..2 * n {
        pw.set_hash_target(images_t[i], images[i]);
    }
    let msg_t = builder.add_virtual_targets(4);
    for i in 0..4 {
        pw.set_target(msg_t[i], msg[i]);
    }
    BoolTarget{
        
    }
    let v_t: Vec<Target> = msg_t
        .iter()
        .map(|&x| builder.split_le_base::<2>(x, 64))
        .flatten()
        .collect();
    for i in 0..n {
        pw.set_target(v_t[i], v[i])
    }
    // dbg!(images.len());
}

fn u64_to_vec(x: u64) -> Vec<bool> {
    let mut x = x;
    let mut v = vec![];
    for _ in 0..64 {
        v.push((x & 1) == 1);
        x >>= 1;
    }
    v
}
