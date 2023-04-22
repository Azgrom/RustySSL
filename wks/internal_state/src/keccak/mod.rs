use crate::keccak::chi::Chi;
use crate::keccak::from_bytes::FromBytes;
use crate::keccak::iota::Iota;
use crate::keccak::pi::Pi;
use crate::keccak::rho::Rho;
use crate::keccak::state::{KeccakState, KeccakStateIter, KeccakStateIterMut};
use crate::keccak::theta::Theta;
use core::mem::size_of;
use core::ops::{BitAnd, BitAndAssign, BitOr, BitXor, BitXorAssign, Not, Range, Sub};
use core::slice::ChunksExact;
use n_bit_words_lib::{LittleEndianBytes, NBitWord, Rotate, TSize};

pub(crate) mod chi;
mod from_bytes;
pub(crate) mod iota;
pub(crate) mod pi;
pub(crate) mod rho;
pub(crate) mod state;
pub(crate) mod theta;

pub(crate) const WIDTH: usize = 5;
pub(crate) const HEIGHT: usize = 5;
const LANES: usize = WIDTH * HEIGHT;

const RC: [u64; 24] = [
    0x0000000000000001,
    0x0000000000008082,
    0x800000000000808A,
    0x8000000080008000,
    0x000000000000808B,
    0x0000000080000001,
    0x8000000080008081,
    0x8000000000008009,
    0x000000000000008A,
    0x0000000000000088,
    0x0000000080008009,
    0x000000008000000A,
    0x000000008000808B,
    0x800000000000008B,
    0x8000000000008089,
    0x8000000000008003,
    0x8000000000008002,
    0x8000000000000080,
    0x000000000000800A,
    0x800000008000000A,
    0x8000000080008081,
    0x8000000000008080,
    0x0000000080000001,
    0x8000000080008008,
];

/// The `KeccakSponge` struct implements a generic sponge construction based on the Keccak permutation.
/// # Sponge construction
///
/// The sponge construction is a cryptographic primitive that can be used to build hash functions,
/// stream ciphers, and more. It is based on an internal state and two operations: absorbing and
/// squeezing. The internal state is divided into two parts: a public part called the rate and a
/// secret part called the capacity. The Keccak sponge construction uses the Keccak-f permutation as
/// its underlying function.
///
/// The Keccak-f permutation is a family of permutations parameterized by the width of the state.
/// The most commonly used instance is Keccak-f[1600], with a state width of 1600 bits.
#[derive(Clone, Debug)]
pub struct KeccakSponge<T, const RATE: usize, const OUTPUT_SIZE: usize>
where
    T: Default + Copy,
{
    /// The internal state of the sponge, which holds a Keccak state of generic type `T` (e.g., u64 for Keccak-f[1600])
    state: KeccakState<T>,
}

impl<T, const RATE: usize, const OUTPUT_SIZE: usize> KeccakSponge<T, RATE, OUTPUT_SIZE>
where
    T: BitAnd
        + BitAndAssign
        + BitOr<NBitWord<T>, Output = NBitWord<T>>
        + BitXor<Output = T>
        + BitXorAssign
        + Copy
        + Default
        + Not<Output = T>,
    NBitWord<T>: From<u64> + LittleEndianBytes + Rotate + TSize<T>,
    u32: Sub<NBitWord<T>, Output = NBitWord<T>>,
{
    /// Creates a new Keccak sponge with the specified rate and capacity
    /// * `N`: The block size, in bytes, of the sponge construction. It is equal to the rate divided by 8.
    pub fn new() -> Self {
        KeccakSponge {
            state: KeccakState::default(),
        }
    }

    /// Absorbs the input data into the sponge
    /// The absorb method takes an input byte slice and processes it through the sponge construction.
    /// It first pads the input using the padding rule, then divides the padded input into blocks of
    /// size `N`. Each block is XORed with the rate portion of the state, followed by the application of
    /// the Keccak-f permutation
    pub fn absorb(&mut self, input: &[u8]) {
        let lanes_to_fulfill = RATE / (u8::BITS as usize * size_of::<T>());
        for (x, byte) in
            KeccakStateIterMut::new(&mut self.state).take(lanes_to_fulfill).zip(input.chunks_exact(size_of::<T>()))
        {
            *x = NBitWord::<T>::from_le_bytes(byte);
        }

        self.state.apply_f();
    }

    /// Squeezes the output data from the sponge
    pub fn squeeze(&mut self) -> [u8; OUTPUT_SIZE] {
        let t_size = size_of::<T>();
        let bytes_to_copy = RATE / u8::BITS as usize;
        let mut output = [0u8; OUTPUT_SIZE];
        let mut remaining_bytes = OUTPUT_SIZE;

        while remaining_bytes > 0 {
            output
                .chunks_mut(t_size)
                .zip(KeccakStateIter::new(&mut self.state).take(bytes_to_copy / t_size))
                .for_each(|(le_bytes, lane)| le_bytes.clone_from_slice(lane.to_le_bytes().as_ref()));

            remaining_bytes =remaining_bytes.saturating_sub(bytes_to_copy);

            if remaining_bytes > 0 {
                self.state.apply_f();
            }
        }

        output
    }
}
