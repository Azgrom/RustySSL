use crate::{Sha384Hasher, BYTES_LEN};
use core::{hash::BuildHasher, ops::AddAssign};
use hash_ctx_lib::ByteArrayWrapper;
use internal_hasher::{GenericPad, HashAlgorithm, U128Size};
use internal_state::{BytesLen, DWords, GenericStateHasher, Sha512BitsState};
use n_bit_words_lib::NBitWord;

const H0: u64 = 0xCBBB9D5DC1059ED8;
const H1: u64 = 0x629A292A367CD507;
const H2: u64 = 0x9159015A3070DD17;
const H3: u64 = 0x152FECD8F70E5939;
const H4: u64 = 0x67332667FFC00B31;
const H5: u64 = 0x8EB44A8768581511;
const H6: u64 = 0xDB0C2E0D64F98FA7;
const H7: u64 = 0x47B5481DBEFA4FA4;

const HX: [u64; 8] = [H0, H1, H2, H3, H4, H5, H6, H7];

/// `Sha384State` represents the state of a SHA-384 hashing process.
///
/// The state holds intermediate hash calculations, allowing you to pause and resume the hashing process.
/// This is particularly useful when working with large data sets or streaming inputs. With a `Sha384State`, hashing can
/// be performed in chunks, thus eliminating the need to hold all the data in memory simultaneously.
///
/// # Example
///
/// This example demonstrates how to persist the state of a SHA-384 hash operation:
///
/// ```rust
/// # use std::hash::{BuildHasher, Hash, Hasher};
/// # use rs_sha384::{Sha384Hasher, Sha384State};
/// let hello = b"hello";
/// let world = b" world";
///
/// let mut default_sha384hasher = Sha384State::default().build_hasher();
/// default_sha384hasher.write(hello);
///
/// let intermediate_state: Sha384State = default_sha384hasher.clone().into();
///
/// default_sha384hasher.write(world);
///
/// let mut from_sha384state: Sha384Hasher = intermediate_state.into();
/// from_sha384state.write(world);
///
/// let default_hello_world_result = default_sha384hasher.finish();
/// let from_arbitrary_state_result = from_sha384state.finish();
/// assert_ne!(default_hello_world_result, from_arbitrary_state_result);
/// ```
///
/// ## Note
/// In this example, even though the internal states of `default_sha384hasher` and `from_sha384state` are identical
/// before the `Hasher::finish` call, the results are different. This is because `from_sha384state` is instantiated with
/// an empty pad, while the `default_sha384hasher`'s pad is already populated with `b"hello"`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Sha384State(
    pub NBitWord<u64>,
    pub NBitWord<u64>,
    pub NBitWord<u64>,
    pub NBitWord<u64>,
    pub NBitWord<u64>,
    pub NBitWord<u64>,
    pub NBitWord<u64>,
    pub NBitWord<u64>,
);

impl AddAssign<Sha512BitsState> for Sha384State {
    fn add_assign(&mut self, rhs: Sha512BitsState) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
        self.3 += rhs.3;
        self.4 += rhs.4;
        self.5 += rhs.5;
        self.6 += rhs.6;
        self.7 += rhs.7;
    }
}

impl BuildHasher for Sha384State {
    type Hasher = Sha384Hasher;

    fn build_hasher(&self) -> Self::Hasher {
        Self::Hasher::default()
    }
}

impl BytesLen for Sha384State {
    fn len() -> usize {
        BYTES_LEN
    }
}

impl Default for Sha384State {
    fn default() -> Self {
        Self::from(HX)
    }
}

impl From<[u8; BYTES_LEN]> for Sha384State {
    fn from(v: [u8; BYTES_LEN]) -> Self {
        Self(
            NBitWord::from(u64::from_ne_bytes([v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7]])),
            NBitWord::from(u64::from_ne_bytes([v[8], v[9], v[10], v[11], v[12], v[13], v[14], v[15]])),
            NBitWord::from(u64::from_ne_bytes([v[16], v[17], v[18], v[19], v[20], v[21], v[22], v[23]])),
            NBitWord::from(u64::from_ne_bytes([v[24], v[25], v[26], v[27], v[28], v[29], v[30], v[31]])),
            NBitWord::from(u64::from_ne_bytes([v[32], v[33], v[34], v[35], v[36], v[37], v[38], v[39]])),
            NBitWord::from(u64::from_ne_bytes([v[40], v[41], v[42], v[43], v[44], v[45], v[46], v[47]])),
            NBitWord::from(u64::default()),
            NBitWord::from(u64::default())
        )
    }
}

impl From<[u64; 8]> for Sha384State {
    fn from(v: [u64; 8]) -> Self {
        Self(
            NBitWord::from(v[0]),
            NBitWord::from(v[1]),
            NBitWord::from(v[2]),
            NBitWord::from(v[3]),
            NBitWord::from(v[4]),
            NBitWord::from(v[5]),
            NBitWord::from(v[6]),
            NBitWord::from(v[7]),
        )
    }
}

impl From<Sha384State> for ByteArrayWrapper<BYTES_LEN> {
    fn from(value: Sha384State) -> Self {
        let a = u64::to_be_bytes(value.0.into());
        let b = u64::to_be_bytes(value.1.into());
        let c = u64::to_be_bytes(value.2.into());
        let d = u64::to_be_bytes(value.3.into());
        let e = u64::to_be_bytes(value.4.into());
        let f = u64::to_be_bytes(value.5.into());

        [
            a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7], c[0], c[1],
            c[2], c[3], c[4], c[5], c[6], c[7], d[0], d[1], d[2], d[3], d[4], d[5], d[6], d[7], e[0], e[1], e[2], e[3],
            e[4], e[5], e[6], e[7], f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7],
        ]
        .into()
    }
}

impl HashAlgorithm for Sha384State {
    type Padding = GenericPad<U128Size, 128, 0x80>;
    type Output = ByteArrayWrapper<BYTES_LEN>;

    fn hash_block(&mut self, bytes: &[u8]) {
        let mut state = Sha512BitsState(
            self.0,
            self.1,
            self.2,
            self.3,
            self.4,
            self.5,
            self.6,
            self.7,
            DWords::<u64>::from(<&[u8; 128]>::try_from(bytes).unwrap()),
        );

        state.block_00_15();
        state.block_16_31();
        state.block_32_47();
        state.block_48_63();
        state.block_64_79();

        *self += state;
    }

    fn state_to_u64(&self) -> u64 {
        Into::<u64>::into(self.0) << 32 | Into::<u64>::into(self.1)
    }
}
