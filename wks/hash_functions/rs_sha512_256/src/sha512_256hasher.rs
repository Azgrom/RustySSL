use crate::Sha512_256State;

#[derive(Clone, Debug)]
pub struct Sha512_256Hasher {
    pub(crate) size: u128,
    pub(crate) state: Sha512_256State,
    pub(crate) padding: [u8; Self::U8_PAD_SIZE as usize],
}
use internal_hasher::{BlockHasher};
impl BlockHasher<u64, u128> for Sha512_256Hasher {
    const U8_PAD_SIZE: u32 = 128;
    const U8_PAD_LAST_INDEX: u32 = Self::U8_PAD_SIZE - 1;
    const U8_PAD_OFFSET: u32 = 111;
    type State = Sha512_256State;

    fn add_assign_size(&mut self, len: usize) {
        self.size += len as u128
    }

    fn add_assign_state(&mut self, state: Self::State) {
        self.state += state
    }

    fn clone_pad_range(&mut self, start: usize, end: usize, bytes: &[u8]) {
        self.padding[start..end].clone_from_slice(bytes)
    }

    fn clone_state(&self) -> Self::State {
        self.state.clone()
    }

    fn get_dw(&self) -> DWords<u64> {
        DWords::<u64>::from(&self.padding)
    }

    fn get_lw(&self) -> usize {
        (self.size & Self::U8_PAD_LAST_INDEX as u128) as usize
    }

    fn get_modulo_pad_size(&self) -> u32 {
        (self.get_size() & Self::U8_PAD_LAST_INDEX as u128) as u32
    }

    fn get_size(&self) -> u128 {
        self.size
    }
}
impl Default for Sha512_256Hasher {
    fn default() -> Self {
        Self {
            size: u128::MIN,
            state: Sha512_256State::default(),
            padding: [0u8; Self::U8_PAD_SIZE as usize],
        }
    }
}
use core::hash::Hash;
impl Hash for Sha512_256Hasher {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.size.hash(state);
        self.state.hash(state);
        self.padding.hash(state);
    }
}
use core::hash::Hasher;
impl Hasher for Sha512_256Hasher {
    fn finish(&self) -> u64 {
        let state = HasherContext::finish(&mut self.clone());
        Into::<u64>::into(state.0.0) << 32 | Into::<u64>::into(state.0.1)
    }

    fn write(&mut self, bytes: &[u8]) {
        BlockHasher::write(self, bytes)
    }
}
use hash_ctx_lib::HasherContext;
use internal_state::DWords;

impl HasherContext<u64, u128> for Sha512_256Hasher {
    fn finish(&mut self) -> Self::State {
        let zero_padding_length = self.zeros_pad_length();
        let mut offset_pad: [u8; Self::U8_PAD_SIZE as usize] = [0u8; Self::U8_PAD_SIZE as usize];
        offset_pad[0] = 0x80;

        let len = self.get_size();
        Hasher::write(self, &offset_pad[..zero_padding_length]);
        Hasher::write(self, &(len * 8).to_be_bytes());

        self.clone_state()
    }
}
