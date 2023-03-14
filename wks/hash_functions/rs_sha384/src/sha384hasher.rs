use crate::sha384state::Sha384State;
use core::hash::{Hash, Hasher};
use hash_ctx_lib::{BlockHasher, HasherContext, HasherWords};

#[derive(Clone)]
pub struct Sha384Hasher {
    pub(crate) size: u128,
    pub(crate) state: Sha384State,
    pub(crate) padding: [u8; Self::U8_PAD_SIZE],
}

impl BlockHasher<u64> for Sha384Hasher {
    const U8_PAD_SIZE: usize = 128;
    const U8_PAD_LAST_INDEX: usize = Self::U8_PAD_SIZE - 1;

    fn zeros_pad_length(size: usize) -> usize {
        1 + (Self::U8_PAD_LAST_INDEX & (111usize.wrapping_sub(size & Self::U8_PAD_LAST_INDEX)))
    }
}

impl Default for Sha384Hasher {
    fn default() -> Self {
        Self {
            size: u128::MIN,
            state: Sha384State::default(),
            padding: [0u8; Self::U8_PAD_SIZE],
        }
    }
}

impl Hash for Sha384Hasher {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.size.hash(state);
        self.state.hash(state);
        self.padding.hash(state);
    }
}

impl Hasher for Sha384Hasher {
    fn finish(&self) -> u64 {
        let state = HasherContext::finish(&mut self.clone());
        Into::<u64>::into(state.0 .0) << 32 | Into::<u64>::into(state.0 .1)
    }

    fn write(&mut self, mut bytes: &[u8]) {
        let len_w = (self.size & Self::U8_PAD_LAST_INDEX as u128) as usize;
        self.size += bytes.len() as u128;

        if len_w != 0 {
            let left = Self::remaining_pad(len_w, &bytes);

            self.padding[len_w..len_w + left].clone_from_slice(&bytes[..left]);

            if Self::incomplete_padding(len_w, left) {
                return;
            }

            Self::hash_block(HasherWords::<u64>::from(&self.padding), &mut self.state);
            bytes = &bytes[left..];
        }

        while bytes.len() >= Self::U8_PAD_SIZE {
            self.padding.clone_from_slice(&bytes[..Self::U8_PAD_SIZE]);
            Self::hash_block(HasherWords::<u64>::from(&self.padding), &mut self.state);
            bytes = &bytes[Self::U8_PAD_SIZE..];
        }

        if !bytes.is_empty() {
            self.padding[..bytes.len()].clone_from_slice(bytes);
        }
    }
}

impl HasherContext for Sha384Hasher {
    type State = Sha384State;

    fn finish(&mut self) -> Self::State {
        let zero_padding_length = Self::zeros_pad_length(self.size as usize);
        let mut offset_pad: [u8; Self::U8_PAD_SIZE] = [0u8; Self::U8_PAD_SIZE];
        offset_pad[0] = 0x80;

        let len = self.size;
        self.write(&offset_pad[..zero_padding_length]);
        self.write(&(len * 8).to_be_bytes());

        self.state.clone()
    }
}
