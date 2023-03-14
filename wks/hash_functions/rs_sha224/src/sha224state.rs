use crate::Sha224Hasher;
use core::{
    fmt::{Formatter, LowerHex, UpperHex},
    hash::{BuildHasher, Hash, Hasher},
    ops::AddAssign,
};
use hash_ctx_lib::{BlockHasher, GenericStateHasher, HasherWords};
use internal_state::{LOWER_HEX_ERR, Sha256BitsState, UPPER_HEX_ERR};

const H0: u32 = 0xC1059ED8;
const H1: u32 = 0x367CD507;
const H2: u32 = 0x3070DD17;
const H3: u32 = 0xF70E5939;
const H4: u32 = 0xFFC00B31;
const H5: u32 = 0x68581511;
const H6: u32 = 0x64F98FA7;
const H7: u32 = 0xBEFA4FA4;

#[derive(Clone)]
pub struct Sha224State(pub(crate) Sha256BitsState);

impl GenericStateHasher<u32> for Sha224State {
    fn block_00_15(&mut self, w: &HasherWords<u32>) {
        self.0.block_00_15(w)
    }

    fn block_16_31(&mut self, w: &mut HasherWords<u32>) {
        self.0.block_16_31(w)
    }

    fn block_32_47(&mut self, w: &mut HasherWords<u32>) {
        self.0.block_32_47(w)
    }

    fn block_48_63(&mut self, w: &mut HasherWords<u32>) {
        self.0.block_48_63(w)
    }

    /// Unused abstraction for SHA-224
    fn block_64_79(&mut self, _w: &mut HasherWords<u32>) {}
}

impl AddAssign for Sha224State {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl BuildHasher for Sha224State {
    type Hasher = Sha224Hasher;

    fn build_hasher(&self) -> Self::Hasher {
        Sha224Hasher {
            size: u64::MIN,
            state: self.clone(),
            padding: [0u8; Sha224Hasher::U8_PAD_SIZE],
        }
    }
}

impl Default for Sha224State {
    fn default() -> Self {
        Self(Sha256BitsState(
            H0.into(),
            H1.into(),
            H2.into(),
            H3.into(),
            H4.into(),
            H5.into(),
            H6.into(),
            H7.into(),
        ))
    }
}

impl From<Sha224State> for [u8; 28] {
    fn from(value: Sha224State) -> Self {
        let a = u32::to_be_bytes(value.0 .0.into());
        let b = u32::to_be_bytes(value.0 .1.into());
        let c = u32::to_be_bytes(value.0 .2.into());
        let d = u32::to_be_bytes(value.0 .3.into());
        let e = u32::to_be_bytes(value.0 .4.into());
        let f = u32::to_be_bytes(value.0 .5.into());
        let g = u32::to_be_bytes(value.0 .6.into());

        [
            a[0], a[1], a[2], a[3], b[0], b[1], b[2], b[3], c[0], c[1], c[2], c[3], d[0], d[1], d[2], d[3], e[0], e[1],
            e[2], e[3], f[0], f[1], f[2], f[3], g[0], g[1], g[2], g[3],
        ]
    }
}

impl Hash for Sha224State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl LowerHex for Sha224State {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        LowerHex::fmt(&self.0 .0, f).expect(LOWER_HEX_ERR);
        LowerHex::fmt(&self.0 .1, f).expect(LOWER_HEX_ERR);
        LowerHex::fmt(&self.0 .2, f).expect(LOWER_HEX_ERR);
        LowerHex::fmt(&self.0 .3, f).expect(LOWER_HEX_ERR);
        LowerHex::fmt(&self.0 .4, f).expect(LOWER_HEX_ERR);
        LowerHex::fmt(&self.0 .5, f).expect(LOWER_HEX_ERR);
        LowerHex::fmt(&self.0 .6, f)
    }
}

impl UpperHex for Sha224State {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        UpperHex::fmt(&self.0 .0, f).expect(UPPER_HEX_ERR);
        UpperHex::fmt(&self.0 .1, f).expect(UPPER_HEX_ERR);
        UpperHex::fmt(&self.0 .2, f).expect(UPPER_HEX_ERR);
        UpperHex::fmt(&self.0 .3, f).expect(UPPER_HEX_ERR);
        UpperHex::fmt(&self.0 .4, f).expect(UPPER_HEX_ERR);
        UpperHex::fmt(&self.0 .5, f).expect(UPPER_HEX_ERR);
        UpperHex::fmt(&self.0 .6, f)
    }
}
