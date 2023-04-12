use crate::Sha384State;
use core::hash::Hasher;
use hash_ctx_lib::{GenericHasher, NewHasherContext};

#[derive(Clone, Debug)]
pub struct Sha384Hasher(GenericHasher<Sha384State>);

impl Default for Sha384Hasher {
    fn default() -> Self {
        Self(GenericHasher::default())
    }
}

impl Hasher for Sha384Hasher {
    fn finish(&self) -> u64 {
        self.0.finish()
    }

    fn write(&mut self, bytes: &[u8]) {
        self.0.write(bytes)
    }
}

impl NewHasherContext for Sha384Hasher {
    type State = Sha384State;

    fn finish(&mut self) -> Self::State {
        NewHasherContext::finish(&mut self.0)
    }
}
