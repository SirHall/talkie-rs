// TODO: Maybe user's want to modify these values?

pub(crate) const FS : u32 = 8_000; // Speech engine sample rate

pub(crate) const tms_energy : [u8; 0x10] = [
    0x00, 0x02, 0x03, 0x04, 0x05, 0x07, 0x0a, 0x0f, 0x14, 0x20, 0x29, 0x39, 0x51, 0x72, 0xa1, 0xff,
];

pub(crate) const tms_period : [u8; 0x40] = [
    0x00, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 0x20, 0x21,
    0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B, 0x2D, 0x2F, 0x31, 0x33, 0x35, 0x36, 0x39, 0x3B, 0x3D,
    0x3F, 0x42, 0x45, 0x47, 0x49, 0x4D, 0x4F, 0x51, 0x55, 0x57, 0x5C, 0x5F, 0x63, 0x66, 0x6A, 0x6E, 0x73, 0x77, 0x7B,
    0x80, 0x85, 0x8A, 0x8F, 0x95, 0x9A, 0xA0,
];

pub(crate) const tms_k1 : [i16; 0x20] = [
    -32064, -31872, -31808, -31680, -31552, -31424, -31232, -30848, -30592, -30336, -30016, -29696, -29376, -28928,
    -28480, -27968, -26368, -24256, -21632, -18368, -14528, -10048, -5184, 0, 5184, 10048, 14528, 18368, 21632, 24256,
    26368, 27968,
];

pub(crate) const tms_k2 : [i16; 0x20] = [
    -20992, -19328, -17536, -15552, -13440, -11200, -8768, -6272, -3712, -1088, 1536, 4160, 6720, 9216, 11584, 13824,
    15936, 17856, 19648, 21248, 22656, 24000, 25152, 26176, 27072, 27840, 28544, 29120, 29632, 30080, 30464, 32384,
];

pub(crate) const tms_k3 : [i8; 0x10] = [-110, -97, -83, -70, -56, -43, -29, -16, -2, 11, 25, 38, 52, 65, 79, 92];
pub(crate) const tms_k4 : [i8; 0x10] = [-82, -68, -54, -40, -26, -12, 1, 15, 29, 43, 57, 71, 85, 99, 113, 126];
pub(crate) const tms_k5 : [i8; 0x10] = [-82, -70, -59, -47, -35, -24, -12, -1, 11, 23, 34, 46, 57, 69, 81, 92];
pub(crate) const tms_k6 : [i8; 0x10] = [-64, -53, -42, -31, -20, -9, 3, 14, 25, 36, 47, 58, 69, 80, 91, 102];
pub(crate) const tms_k7 : [i8; 0x10] = [-77, -65, -53, -41, -29, -17, -5, 7, 19, 31, 43, 55, 67, 79, 90, 102];
pub(crate) const tms_k8 : [i8; 0x08] = [-64, -40, -16, 7, 31, 55, 79, 102];
pub(crate) const tms_k9 : [i8; 0x08] = [-64, -44, -24, -4, 16, 37, 57, 77];
pub(crate) const tms_k10 : [i8; 0x08] = [-51, -33, -15, 4, 22, 32, 59, 77];

pub(crate) const CHIRP_SIZE : u8 = 41;
pub(crate) const chirp : [i8; CHIRP_SIZE as usize] = [
    0, 42, -44, 50, -78, 18, 37, 20, 2, -31, -59, 2, 95, 90, 5, 15, 38, -4, -91, -91, -42, -35, -36, -4, 37, 43, 34,
    33, 15, -1, -8, -18, -19, -17, -9, -10, -6, 0, 3, 2, 1,
];