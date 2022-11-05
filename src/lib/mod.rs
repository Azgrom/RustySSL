use core::ops::{BitOr, Shl, Shr};
use std::ops::Range;

fn main() {
    println!("Hello, world!");
}

const T_0_15: u32 = 0x5a827999;
const T_16_19: u32 = T_0_15;
const T_20_39: u32 = 0x6ed9eba1;
const T_40_59: u32 = 0x8f1bbcdc;
const T_60_79: u32 = 0xca62c1d6;
const U8_TO_U32: [u8; 4] = [0, 8, 16, 24];

trait ShiftSideways:
    Shl<Output = Self> + Shr<Output = Self> + BitOr<Output = Self> + Copy + Sized
{
}

impl ShiftSideways for u32 {}

fn rotate<R: ShiftSideways>(x: R, l: R, r: R) -> R {
    (x << l) | (x >> r)
}

fn rotate_left(x: u32, n: u32) -> u32 {
    rotate(x, n, 32 - n)
}

fn rotate_right(x: u32, n: u32) -> u32 {
    rotate(x, 32 - n, n)
}

fn get_dst_range<T>(src: &[T], dest: &[u32], dest_offset: usize) -> Range<usize> {
    let dst_start = dest_offset & (dest.len() - 1);
    let dst_end = dst_start + src.len();

    Range {
        start: dst_start,
        end: dst_end
    }
}

fn get_src_range<T>(src: &[T], src_slice_len: usize) -> Range<usize> {
    if src_slice_len > src.len() {
        return Range {
            start: 0,
            end: src.len()
        }
    }

    Range {
        start: 0,
        end: (src_slice_len & (src.len() - 1)) % src.len()
    }
}

pub trait Sha1 {
    fn init() -> Self;
    fn update(&mut self, data_in: &[u8], len: usize);
    fn finalize(&mut self) -> [u8; 20];
}

pub struct ShaContext {
    size: usize,
    h: [u32; 5],
    w: [u32; 16]
}

trait MemoryCopy<S, D> {
    /// Copies n bytes from src to memory dest, using a reference receiving point in dest
    fn mem_cpy(src: &[S], dest: &mut [D], src_slice_len: usize, dest_offset:usize);
}

trait ShaSource<T> {
    fn src(i: usize, v: &[T]) -> u32;
    fn t_0_15(t: u8, block: &[T], a: u32, b: &mut u32, c: u32, d: u32, e: &mut u32, array: &mut [u32]);
    fn block(h: &mut [u32; 5], block: &[T]);
    fn process(&mut self, data_in: &[T], len: usize);
}

impl MemoryCopy<u8, u8> for ShaContext {
    fn mem_cpy(src: &[u8], dest: &mut [u8], src_slice_len: usize, dest_offset: usize) {
        dest[dest_offset..].clone_from_slice(&src[..src_slice_len])
    }
}

impl MemoryCopy<u8, u32> for ShaContext {
    fn mem_cpy(src: &[u8], dest: &mut [u32], src_slice_len: usize, mut dest_offset: usize) {
        // TODO: this impl considers src and src_len larger than 4 bytes. Implement flows for lesser cases
        let src_range = get_src_range(src, src_slice_len);
        // TODO: Problema atual: indices até 4 quando fim do vetor pode ser até 1
        let u32_src = &src[src_range].chunks(4).map(|c| {
            match c.len() {
                4 => (c[0] as u32) | ((c[1] as u32) << 8) | ((c[2] as u32) << 16) | ((c[3] as u32) << 24),
                3 => (c[0] as u32) | ((c[1] as u32) << 8) | ((c[2] as u32) << 16),
                2 => (c[0] as u32) | ((c[1] as u32) << 8),
                1 => c[0] as u32,
                _ => panic!("")
            }
        }).collect::<Vec<u32>>();

        let mut dst_range = get_dst_range(src, dest, dest_offset);
        dst_range.start = (dst_range.start / 4) + (dst_range.start % 4);
        dst_range.end = dst_range.start + u32_src.len();
        &dest[dst_range].clone_from_slice(&u32_src);
    }
}

impl MemoryCopy<u32, u32> for ShaContext {
    fn mem_cpy(src: &[u32], dest: &mut [u32], src_slice_len: usize, dest_offset: usize) {
        // TODO: Validar se intervalo dos vetor receptor é igual ou maio que do vetor fonte
        let src_range = get_src_range(src, src_slice_len);
        let dst_range = get_dst_range(src, dest, dest_offset);

        dest[dst_range].clone_from_slice(&src[src_range])
    }
}

impl ShaSource<u8> for ShaContext {
    fn src(i: usize, v: &[u8]) -> u32 {
        // TODO: See if there should have validation here
        let s = i * 4;
        ((v[s] as u32) << 24)
            | ((v[s + 1] as u32) << 16)
            | ((v[s + 2] as u32) << 8)
            | (v[s + 3] as u32)
    }

    fn t_0_15(t: u8, block: &[u8], a: u32, b: &mut u32, c: u32, d: u32, e: &mut u32, array: &mut [u32])  {
        let temp = Self::src(t as usize, block);
        Self::set_w(t as usize, temp, array);
        *e += temp + rotate_left(a, 5) + Self::f1(*b, c, d) + T_0_15;
        *b = rotate_right(*b, 2);
    }

    fn block(h: &mut [u32; 5], block: &[u8]) {
        let mut a = h[0];
        let mut b = h[1];
        let mut c = h[2];
        let mut d = h[3];
        let mut e = h[4];

        let mut array: [u32; 16] = [0; 16];

        /* Round 1 - iterations 0-16 take their input from 'block' */
        Self::t_0_15(0, block, a, &mut b, c, d, &mut e, &mut array);
        Self::t_0_15(1, block, e, &mut a, b, c, &mut d, &mut array);
        Self::t_0_15(2, block, d, &mut e, a, b, &mut c, &mut array);
        Self::t_0_15(3, block, c, &mut d, e, a, &mut b, &mut array);
        Self::t_0_15(4, block, b, &mut c, d, e, &mut a, &mut array);
        Self::t_0_15(5, block, a, &mut b, c, d, &mut e, &mut array);
        Self::t_0_15(6, block, e, &mut a, b, c, &mut d, &mut array);
        Self::t_0_15(7, block, d, &mut e, a, b, &mut c, &mut array);
        Self::t_0_15(8, block, c, &mut d, e, a, &mut b, &mut array);
        Self::t_0_15(9, block, b, &mut c, d, e, &mut a, &mut array);
        Self::t_0_15(10, block, a, &mut  b, c, d, &mut  e, &mut array);
        Self::t_0_15(11, block, e, &mut  a, b, c, &mut  d, &mut array);
        Self::t_0_15(12, block, d, &mut  e, a, b, &mut  c, &mut array);
        Self::t_0_15(13, block, c, &mut  d, e, a, &mut  b, &mut array);
        Self::t_0_15(14, block, b, &mut  c, d, e, &mut  a, &mut array);
        Self::t_0_15(15, block, a, &mut  b, c, d, &mut  e, &mut array);

        /* Round 1 - tail. Input from 512-bit mixing array */
        Self::t_16_19(16, &mut array, e, &mut a, b, c, &mut d);
        Self::t_16_19(17, &mut array, d, &mut e, a, b, &mut c);
        Self::t_16_19(18, &mut array, c, &mut d, e, a, &mut b);
        Self::t_16_19(19, &mut array, b, &mut c, d, e, &mut a);

        /* Round 2 */
        Self::t_20_39(20, &mut array, a, b, c, d, e);
        Self::t_20_39(21, &mut array, e, a, b, c, d);
        Self::t_20_39(22, &mut array, d, e, a, b, c);
        Self::t_20_39(23, &mut array, c, d, e, a, b);
        Self::t_20_39(24, &mut array, b, c, d, e, a);
        Self::t_20_39(25, &mut array, a, b, c, d, e);
        Self::t_20_39(26, &mut array, e, a, b, c, d);
        Self::t_20_39(27, &mut array, d, e, a, b, c);
        Self::t_20_39(28, &mut array, c, d, e, a, b);
        Self::t_20_39(29, &mut array, b, c, d, e, a);
        Self::t_20_39(30, &mut array, a, b, c, d, e);
        Self::t_20_39(31, &mut array, e, a, b, c, d);
        Self::t_20_39(32, &mut array, d, e, a, b, c);
        Self::t_20_39(33, &mut array, c, d, e, a, b);
        Self::t_20_39(34, &mut array, b, c, d, e, a);
        Self::t_20_39(35, &mut array, a, b, c, d, e);
        Self::t_20_39(36, &mut array, e, a, b, c, d);
        Self::t_20_39(37, &mut array, d, e, a, b, c);
        Self::t_20_39(38, &mut array, c, d, e, a, b);
        Self::t_20_39(39, &mut array, b, c, d, e, a);

        /* Round 3 */
        Self::t_40_59(40, &mut array, a, b, c, d, e);
        Self::t_40_59(41, &mut array, e, a, b, c, d);
        Self::t_40_59(42, &mut array, d, e, a, b, c);
        Self::t_40_59(43, &mut array, c, d, e, a, b);
        Self::t_40_59(44, &mut array, b, c, d, e, a);
        Self::t_40_59(45, &mut array, a, b, c, d, e);
        Self::t_40_59(46, &mut array, e, a, b, c, d);
        Self::t_40_59(47, &mut array, d, e, a, b, c);
        Self::t_40_59(48, &mut array, c, d, e, a, b);
        Self::t_40_59(49, &mut array, b, c, d, e, a);
        Self::t_40_59(50, &mut array, a, b, c, d, e);
        Self::t_40_59(51, &mut array, e, a, b, c, d);
        Self::t_40_59(52, &mut array, d, e, a, b, c);
        Self::t_40_59(53, &mut array, c, d, e, a, b);
        Self::t_40_59(54, &mut array, b, c, d, e, a);
        Self::t_40_59(55, &mut array, a, b, c, d, e);
        Self::t_40_59(56, &mut array, e, a, b, c, d);
        Self::t_40_59(57, &mut array, d, e, a, b, c);
        Self::t_40_59(58, &mut array, c, d, e, a, b);
        Self::t_40_59(59, &mut array, b, c, d, e, a);

        /* Round 4 */
        Self::t_60_79(60, &mut array, a, b, c, d, e);
        Self::t_60_79(61, &mut array, e, a, b, c, d);
        Self::t_60_79(62, &mut array, d, e, a, b, c);
        Self::t_60_79(63, &mut array, c, d, e, a, b);
        Self::t_60_79(64, &mut array, b, c, d, e, a);
        Self::t_60_79(65, &mut array, a, b, c, d, e);
        Self::t_60_79(66, &mut array, e, a, b, c, d);
        Self::t_60_79(67, &mut array, d, e, a, b, c);
        Self::t_60_79(68, &mut array, c, d, e, a, b);
        Self::t_60_79(69, &mut array, b, c, d, e, a);
        Self::t_60_79(70, &mut array, a, b, c, d, e);
        Self::t_60_79(71, &mut array, e, a, b, c, d);
        Self::t_60_79(72, &mut array, d, e, a, b, c);
        Self::t_60_79(73, &mut array, c, d, e, a, b);
        Self::t_60_79(74, &mut array, b, c, d, e, a);
        Self::t_60_79(75, &mut array, a, b, c, d, e);
        Self::t_60_79(76, &mut array, e, a, b, c, d);
        Self::t_60_79(77, &mut array, d, e, a, b, c);
        Self::t_60_79(78, &mut array, c, d, e, a, b);
        Self::t_60_79(79, &mut array, b, c, d, e, a);

        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
    }

    fn process(&mut self, mut data_in: &[u8], mut len: usize) {
        let mut len_w = self.size & 63;

        self.size += len;

        if len_w > 0 {
            let mut left = 64 - len_w;
            if len < left {
                left = len;
            }

            Self::mem_cpy(data_in, &mut self.w, left, len_w);

            len_w = (len_w + left) & 63;
            len -= left;
            data_in = &data_in[(left & 7)..];

            if len_w > 0 {
                return;
            }

            Self::block(&mut self.h, &mut self.w);
        }

        while len >= 64 {
            Self::block(&mut self.h, data_in);
            data_in = &data_in[64..];
            len -= 64;
        }

        if len > 0 {
            Self::mem_cpy(data_in, &mut self.w, len, 0);
        }
    }
}

impl ShaSource<u32> for ShaContext {
    fn src(i: usize, v: &[u32]) -> u32 {
        v[i].to_be()
    }

    fn t_0_15(t: u8, block: &[u32], a: u32, b: &mut u32, c: u32, d: u32, e: &mut u32, array: &mut [u32]) {
        let temp = Self::src(t as usize, block);
        Self::set_w(t as usize, temp, array);
        *e = (*e).wrapping_add(temp.wrapping_add(rotate_left(a, 5).wrapping_add(Self::f1(*b, c, d).wrapping_add(T_0_15))));
        *b = rotate_right(*b, 2);
    }

    fn block(h: &mut [u32; 5], block: &[u32]) {
        let mut a = h[0];
        let mut b = h[1];
        let mut c = h[2];
        let mut d = h[3];
        let mut e = h[4];

        let mut array: [u32; 16] = [0; 16];

        /* Round 1 - iterations 0-16 take their input from 'block' */
        Self::t_0_15(0, block, a, &mut b, c, d, &mut e, &mut array);
        Self::t_0_15(1, block, e, &mut a, b, c, &mut d, &mut array);
        Self::t_0_15(2, block, d, &mut e, a, b, &mut c, &mut array);
        Self::t_0_15(3, block, c, &mut d, e, a, &mut b, &mut array);
        Self::t_0_15(4, block, b, &mut c, d, e, &mut a, &mut array);
        Self::t_0_15(5, block, a, &mut b, c, d, &mut e, &mut array);
        Self::t_0_15(6, block, e, &mut a, b, c, &mut d, &mut array);
        Self::t_0_15(7, block, d, &mut e, a, b, &mut c, &mut array);
        Self::t_0_15(8, block, c, &mut d, e, a, &mut b, &mut array);
        Self::t_0_15(9, block, b, &mut c, d, e, &mut a, &mut array);
        Self::t_0_15(10, block, a,&mut  b, c, d,&mut  e, &mut array);
        Self::t_0_15(11, block, e,&mut  a, b, c,&mut  d, &mut array);
        Self::t_0_15(12, block, d,&mut  e, a, b,&mut  c, &mut array);
        Self::t_0_15(13, block, c,&mut  d, e, a,&mut  b, &mut array);
        Self::t_0_15(14, block, b,&mut  c, d, e,&mut  a, &mut array);
        Self::t_0_15(15, block, a,&mut  b, c, d,&mut  e, &mut array);

        /* Round 1 - tail. Input from 512-bit mixing array */
        Self::t_16_19(16, &mut array, e, &mut a, b, c, &mut d);
        Self::t_16_19(17, &mut array, d, &mut e, a, b, &mut c);
        Self::t_16_19(18, &mut array, c, &mut d, e, a, &mut b);
        Self::t_16_19(19, &mut array, b, &mut c, d, e, &mut a);

        /* Round 2 */
        Self::t_20_39(20, &mut array, a, b, c, d, e);
        Self::t_20_39(21, &mut array, e, a, b, c, d);
        Self::t_20_39(22, &mut array, d, e, a, b, c);
        Self::t_20_39(23, &mut array, c, d, e, a, b);
        Self::t_20_39(24, &mut array, b, c, d, e, a);
        Self::t_20_39(25, &mut array, a, b, c, d, e);
        Self::t_20_39(26, &mut array, e, a, b, c, d);
        Self::t_20_39(27, &mut array, d, e, a, b, c);
        Self::t_20_39(28, &mut array, c, d, e, a, b);
        Self::t_20_39(29, &mut array, b, c, d, e, a);
        Self::t_20_39(30, &mut array, a, b, c, d, e);
        Self::t_20_39(31, &mut array, e, a, b, c, d);
        Self::t_20_39(32, &mut array, d, e, a, b, c);
        Self::t_20_39(33, &mut array, c, d, e, a, b);
        Self::t_20_39(34, &mut array, b, c, d, e, a);
        Self::t_20_39(35, &mut array, a, b, c, d, e);
        Self::t_20_39(36, &mut array, e, a, b, c, d);
        Self::t_20_39(37, &mut array, d, e, a, b, c);
        Self::t_20_39(38, &mut array, c, d, e, a, b);
        Self::t_20_39(39, &mut array, b, c, d, e, a);

        /* Round 3 */
        Self::t_40_59(40, &mut array, a, b, c, d, e);
        Self::t_40_59(41, &mut array, e, a, b, c, d);
        Self::t_40_59(42, &mut array, d, e, a, b, c);
        Self::t_40_59(43, &mut array, c, d, e, a, b);
        Self::t_40_59(44, &mut array, b, c, d, e, a);
        Self::t_40_59(45, &mut array, a, b, c, d, e);
        Self::t_40_59(46, &mut array, e, a, b, c, d);
        Self::t_40_59(47, &mut array, d, e, a, b, c);
        Self::t_40_59(48, &mut array, c, d, e, a, b);
        Self::t_40_59(49, &mut array, b, c, d, e, a);
        Self::t_40_59(50, &mut array, a, b, c, d, e);
        Self::t_40_59(51, &mut array, e, a, b, c, d);
        Self::t_40_59(52, &mut array, d, e, a, b, c);
        Self::t_40_59(53, &mut array, c, d, e, a, b);
        Self::t_40_59(54, &mut array, b, c, d, e, a);
        Self::t_40_59(55, &mut array, a, b, c, d, e);
        Self::t_40_59(56, &mut array, e, a, b, c, d);
        Self::t_40_59(57, &mut array, d, e, a, b, c);
        Self::t_40_59(58, &mut array, c, d, e, a, b);
        Self::t_40_59(59, &mut array, b, c, d, e, a);

        /* Round 4 */
        Self::t_60_79(60, &mut array, a, b, c, d, e);
        Self::t_60_79(61, &mut array, e, a, b, c, d);
        Self::t_60_79(62, &mut array, d, e, a, b, c);
        Self::t_60_79(63, &mut array, c, d, e, a, b);
        Self::t_60_79(64, &mut array, b, c, d, e, a);
        Self::t_60_79(65, &mut array, a, b, c, d, e);
        Self::t_60_79(66, &mut array, e, a, b, c, d);
        Self::t_60_79(67, &mut array, d, e, a, b, c);
        Self::t_60_79(68, &mut array, c, d, e, a, b);
        Self::t_60_79(69, &mut array, b, c, d, e, a);
        Self::t_60_79(70, &mut array, a, b, c, d, e);
        Self::t_60_79(71, &mut array, e, a, b, c, d);
        Self::t_60_79(72, &mut array, d, e, a, b, c);
        Self::t_60_79(73, &mut array, c, d, e, a, b);
        Self::t_60_79(74, &mut array, b, c, d, e, a);
        Self::t_60_79(75, &mut array, a, b, c, d, e);
        Self::t_60_79(76, &mut array, e, a, b, c, d);
        Self::t_60_79(77, &mut array, d, e, a, b, c);
        Self::t_60_79(78, &mut array, c, d, e, a, b);
        Self::t_60_79(79, &mut array, b, c, d, e, a);

        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
    }

    fn process(&mut self, mut data_in: &[u32], mut len: usize) {
        let mut len_w = self.size & 63;

        self.size += len;

        if len_w > 0 {
            let mut left = 64 - len_w;
            if len < left {
                left = len;
            }

            Self::mem_cpy(data_in, &mut self.w, left, len_w);

            len_w = (len_w + left) & 63;
            len -= left;
            data_in = &data_in[(left & 7)..];

            if len_w > 0 {
                return;
            }

            Self::block(&mut self.h, &mut self.w);
        }

        while len >= 64 {
            Self::block(&mut self.h, data_in);
            data_in = &data_in[64..];
            len -= 64;
        }

        if len > 0 {
            Self::mem_cpy(data_in, &mut self.w, len, 0);
        }
    }
}

impl ShaContext {
    fn set_w(i: usize, val: u32, array: &mut [u32]) {
        array[i & 15] = val;
    }

    fn mix(i: usize, array: &[u32]) -> u32 {
        let x = array[(i + 13) & 15];
        let y = array[(i + 8) & 15];
        let z = array[(i + 2) & 15];
        let t = array[i & 15];

        rotate_left(x ^ y ^ z ^ t, 1)
    }

    fn f1(b: u32, c: u32, d: u32) -> u32 {
        ((c ^ d) & b) ^ d
    }

    fn f2(b: u32, c: u32, d: u32) -> u32 {
        b ^ c ^ d
    }

    fn f3(b: u32, c: u32, d: u32) -> u32 {
        (b & c) + (d & (b ^ c))
    }

    fn f4(b: u32, c: u32, d: u32) -> u32 {
        Self::f2(b, c, d)
    }

    fn zero_first_twenty_four_bits(h: &u32) -> u8 {
        ((*h << 24) >> 24) as u8
    }

    fn round(
        t: u8,
        block: &mut [u32],
        f_n: u32,
        constant: u32,
        a: u32,
        b: &mut u32,
        c: u32,
        d: u32,
        e: &mut u32,
    ) {
        let temp = Self::mix(t as usize, block);
        Self::set_w(t as usize, temp, block);
        *e = (*e).wrapping_add(temp.wrapping_add(rotate_left(a, 5).wrapping_add(f_n.wrapping_add(constant))));
        *b = rotate_right(*b, 2);
    }

    fn t_16_19(t: u8, shamble_arr: &mut [u32], a: u32, b: &mut u32, c: u32, d: u32, e: &mut u32) {
        Self::round(
            t,
            shamble_arr,
            Self::f1(*b, c, d),
            T_16_19,
            a,
            b,
            c,
            d,
            e,
        )
    }

    fn t_20_39(t: u8, shamble_arr: &mut [u32], a: u32, mut b: u32, c: u32, d: u32, mut e: u32) {
        Self::round(
            t,
            shamble_arr,
            Self::f2(b, c, d),
            T_20_39,
            a,
            &mut b,
            c,
            d,
            &mut e,
        )
    }

    fn t_40_59(t: u8, shamble_arr: &mut [u32], a: u32, mut b: u32, c: u32, d: u32, mut e: u32) {
        Self::round(
            t,
            shamble_arr,
            Self::f3(b, c, d),
            T_40_59,
            a,
            &mut b,
            c,
            d,
            &mut e,
        )
    }

    fn t_60_79(t: u8, shamble_arr: &mut [u32], a: u32, mut b: u32, c: u32, d: u32, mut e: u32) {
        Self::round(
            t,
            shamble_arr,
            Self::f4(b, c, d),
            T_60_79,
            a,
            &mut b,
            c,
            d,
            &mut e,
        )
    }
}

impl Sha1 for ShaContext {
    fn init() -> Self {
        let mut ctx = Self {
            size: 0,
            h: [0; 5],
            w: [0; 16]
        };
        /* Initialize H with the magic constants (see FIPS180 for constants) */
        ctx.h[0] = 0x67452301;
        ctx.h[1] = 0xefcdab89;
        ctx.h[2] = 0x98badcfe;
        ctx.h[3] = 0x10325476;
        ctx.h[4] = 0xc3d2e1f0;

        return ctx;
    }

    fn update(&mut self, data_in: &[u8], len: usize) {
        self.process(data_in, len)
    }

    fn finalize(&mut self) -> [u8; 20] {
        let mut pad: [u8; 64] = [0; 64];
        let mut pad_len: [u32; 2]= [0; 2];
        pad[0] = 0x80;

        let i = self.size & 63;
        self.process(&pad, 1 + (63 & (55 - i)));
        self.process(&pad_len, 8);

        let mut hash_out: [u8; 20] = [0; 20];

        self.h.iter().zip((0..5).into_iter()).for_each(|(h, i)| {
            hash_out[0 + (i * 4)] = (*h >> 24) as u8;
            hash_out[1 + (i * 4)] = (*h >> 16) as u8;
            hash_out[2 + (i * 4)] = (*h >> 8) as u8;
            hash_out[3 + (i * 4)] = Self::zero_first_twenty_four_bits(h);
        });

        return hash_out;
    }
}

#[cfg(test)]
mod test {
    use crate::{rotate_left, rotate_right, Sha1, ShaContext};

    #[test]
    fn custom_right_bit_rotation_should_return_same_as_standard_impl() {
        let x: u32 = 5;
        let y: u32 = 2;
        let std_rotate_right = x.rotate_right(y);
        let cus_rotate_right = rotate_right(x, y);

        assert_eq!(std_rotate_right, cus_rotate_right);
    }

    #[test]
    fn custom_left_bit_rotation_should_return_same_as_standard_impl() {
        let x: u32 = 5;
        let y: u32 = 2;
        let std_rotate_left = x.rotate_left(y);
        let cus_rotate_left = rotate_left(x, y);

        assert_eq!(std_rotate_left, cus_rotate_left);
    }

    #[test]
    fn test_commonly_known_sha1_phrases() {
        const PHRASE: &str = "The quick brown fox jumps over the lazy dog";
        let mut ctx = ShaContext::init();
        ctx.update(PHRASE.as_bytes(), PHRASE.len());
        let hash = ctx.finalize();
        let hash_hex_str = hash.iter().map(|b| format!("{:02x}", b)).collect::<String>();

        assert_eq!(hash_hex_str, "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12");
    }
}
