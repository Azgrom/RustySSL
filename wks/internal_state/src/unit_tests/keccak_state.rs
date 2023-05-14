use crate::{
    keccak::{chi::Chi, iota::Iota, pi::Pi, rho::Rho, theta::Theta, HEIGHT, WIDTH},
    KeccakState,
};

// Test constants, based on examples from the Keccak reference
const INITIAL_STATE: [[u64; WIDTH]; HEIGHT] = [
    [0x0000000000000001, 0x0000000000008082, 0x800000000000808A, 0x8000000080008000, 0x000000000000808B],
    [0x0000000000000197, 0x8000000080008081, 0x8000000000008009, 0x000000000000008A, 0x0000000000000088],
    [0x0000000080008009, 0x000000008000000A, 0x000000008000808B, 0x800000000000008B, 0x8000000000008089],
    [0x8000000000008003, 0x8000000000008002, 0x8000000000000080, 0x000000000000800A, 0x800000008000000A],
    [0x8000000080008081, 0x8000000000008080, 0x0000000080000001, 0x8000000080008008, 0x0000000000000001],
];
// Test constants, after theta step
const AFTER_THETA_STATE: [[u64; WIDTH]; HEIGHT] = [
    [0x0000000080000197, 0x000000000001008C, 0x0000000000018006, 0x000000018000018B, 0x80000000000102B2],
    [0x0000000080000001, 0x800000008001008F, 0x0000000000018085, 0x8000000100008101, 0x80000000000182B1],
    [0x000000000000819F, 0x0000000080018004, 0x8000000080018007, 0x0000000100008100, 0x00000000000102B0],
    [0x8000000080008195, 0x800000000001000C, 0x000000000001000C, 0x8000000100000181, 0x0000000080018233],
    [0x8000000000008117, 0x800000000001008E, 0x800000008001008D, 0x0000000180000183, 0x8000000000018238],
];
// Test constants, after rho step
const AFTER_RHO_STATE: [[u64; WIDTH]; HEIGHT] = [
    [0x0000000000000001, 0x0000000000010104, 0xA000000000002022, 0x0800080008000000, 0x0000040458000000],
    [0x0000197000000000, 0x0808180000000800, 0x0000000000200260, 0x4500000000000000, 0x0000000008800000],
    [0x0000000400040048, 0x0000020000002800, 0x0404580000000400, 0x0000000117000000, 0x004044C000000000],
    [0x0100070000000000, 0x1000500000000000, 0x0000000000404000, 0x0000001001400000, 0x0000008000000A80],
    [0x0002000202060000, 0x0000000000020202, 0x2000000010000000, 0x0880000000800080, 0x0000000000004000],
];
// Test constants, after pi step
const AFTER_PI_STATE: [[u64; WIDTH]; HEIGHT] = [
    [0x0000000000000001, 0x8000000080008081, 0x000000008000808B, 0x000000000000800A, 0x0000000000000001],
    [0x8000000080008000, 0x0000000000000088, 0x0000000080008009, 0x8000000000008002, 0x0000000080000001],
    [0x0000000000008082, 0x8000000000008009, 0x800000000000008B, 0x800000008000000A, 0x8000000080008081],
    [0x000000000000808B, 0x0000000000000197, 0x000000008000000A, 0x8000000000000080, 0x8000000080008008],
    [0x800000000000808A, 0x000000000000008A, 0x8000000000008089, 0x8000000000008003, 0x8000000000008080],
];
// Test constants, after chi step
const AFTER_CHI_STATE: [[u64; WIDTH]; HEIGHT] = [
    [0x8000000000000009, 0x0000000080008082, 0x8000000000008001, 0x8000000080008000, 0x0000000000000009],
    [0x000000000000019F, 0x8000000080008003, 0x8000000000008009, 0x000000000000019D, 0x8000000080008088],
    [0x0000000080000088, 0x800000008000000A, 0x000000008000008B, 0x800000008000008B, 0x800000000000808B],
    [0x8000000000008083, 0x8000000000000008, 0x0000000080000080, 0x000000000000000B, 0x800000008000000A],
    [0x8000000000008080, 0x0000000000000088, 0x0000000080000000, 0x0000000000000088, 0x0000000000000001],
];
// Test constants, after first iota step
const AFTER_IOTA_STATE_ON_FIRST_CYCLE: [[u64; WIDTH]; HEIGHT] = [
    [0x0C00040080000596, 0x1028F80030300800, 0x0C003C00408E2400, 0x00200000B0300197, 0x1008F800608E2800],
    [0x18000018B0040CF8, 0x200190182B180000, 0x9000000010042CE9, 0x28019018A0000000, 0xB00000001B182011],
    [0x0200010200020118, 0x0000008001E21240, 0x02000100045C0000, 0x0000008001823218, 0x00000002043E2040],
    [0x00000A1592001000, 0x0000001080060008, 0x8300020007801001, 0x0000081514060000, 0x8300000001800009],
    [0x8001580000006001, 0x81C2230000800140, 0x008158000004023A, 0x81032B0000006101, 0x00C000000084027A],
];
// Test constants, after second iota step
const AFTER_IOTA_STATE_ON_SECOND_CYCLE: [[u64; WIDTH]; HEIGHT] = [
    [0x0D3EEC1E9B581D27, 0x86DA7850A21230EB, 0xC0B03F4B596835C3, 0xAA2D2A9910FB5A96, 0xA7A5808F08878322],
    [0x4131F4724513A02A, 0x502DAF2805004285, 0x64B7FA9851B4EE5B, 0x883975C076A0F8E4, 0x6461E49A70244584],
    [0x7A02D2477DB48B84, 0xCB8BA6A06826F71C, 0xD51F3868DCFD2157, 0x020F504E3186E385, 0x5DC40C71F0CB3464],
    [0x9D6F84530CB1492C, 0xB2084FF7D7E6C0FC, 0x8EF42CB901DA449C, 0x1749EF115F0B7557, 0xB5DB1B1885E8BABC],
    [0xFBAB9464A4626E85, 0x63216E83A2C700B7, 0x504798FE900FFA6E, 0x20F1628098A88931, 0xB316E0BF1A8C336E],
];

#[test]
fn assert_theta_correctness() {
    let mut state = KeccakState::from(INITIAL_STATE);
    state.theta();

    assert_eq!(state, KeccakState::from(AFTER_THETA_STATE));
}

#[test]
fn assert_rho_correctness() {
    let mut state = KeccakState::from(INITIAL_STATE);
    state.rho();

    assert_eq!(state, KeccakState::from(AFTER_RHO_STATE));
}

#[test]
fn assert_pi_correctness() {
    let mut state = KeccakState::from(INITIAL_STATE);
    state.pi();

    assert_eq!(state, KeccakState::from(AFTER_PI_STATE));
}

#[test]
fn assert_chi_correctness() {
    let mut state = KeccakState::from(INITIAL_STATE);
    state.chi();

    assert_eq!(state, KeccakState::from(AFTER_CHI_STATE));
}

#[test]
fn assert_iota_first_cycle_correctness() {
    let mut state = KeccakState::from(INITIAL_STATE);
    state.theta();
    state.rho();
    state.pi();
    state.chi();
    state.iota(0);

    assert_eq!(state, KeccakState::from(AFTER_IOTA_STATE_ON_FIRST_CYCLE));
}

#[test]
fn assert_iota_two_cycles_correctness() {
    let mut state = KeccakState::from(INITIAL_STATE);

    for i in 0..2 {
        state.theta();
        state.rho();
        state.pi();
        state.chi();
        state.iota(i);
    }

    assert_eq!(state, KeccakState::from(AFTER_IOTA_STATE_ON_SECOND_CYCLE));
}
