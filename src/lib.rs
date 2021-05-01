#![crate_type = "lib"]
#![crate_name = "keccak_rust"]

mod keccak_f;
use keccak_f::*;

mod sponge;
use sponge::*;

pub type BitsWidth = usize;

pub type Rate = usize;
pub type Capacity = usize;
pub type Security = (Rate, Capacity);

pub type Bit = u8;
pub type State = [[u64; 5]; 5];
pub type BitsVec = Vec<Bit>;
pub type BitsArr = [Bit];

pub type PermutationsNum = usize;

// SHA224, SHA256, SHA384, SHA512
static RATES: [usize; 4] = [1152, 1088, 832, 576];
static CAPACITIES: [usize; 4] = [448, 512, 768, 1024];

// F25, F50, F100, F200, F400, F800, F1600
static PERMUTATIONS_NUM: [usize; 7] = [12, 14, 16, 18, 20, 22, 24];

static ROUND_CONSTANTS: [u64; 24] = [
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

static ROTATION_CONSTANTS: State = [
    [0, 36, 3, 41, 18],
    [1, 44, 10, 45, 2],
    [62, 6, 43, 15, 61],
    [28, 55, 25, 21, 56],
    [27, 20, 39, 8, 14]
];

pub enum StateBitsWidth {
    F25,
    F50,
    F100,
    F200,
    F400,
    F800,
    F1600,
    Custom(BitsWidth)
}

pub enum SecurityLevel {
    SHA224,
    SHA256,
    SHA384,
    SHA512,
    Custom(Security)
}

/// An implementation of keccak functions. [`The Keccak reference`].
///
/// # Example
///
/// ```toml
/// [dependencies]
/// keccak-rust = *
/// ```
/// 
/// ```rust
/// extern crate keccak_rust;
/// use keccak_rust::*;
/// 
/// const YOUR_INPUT_BYTES: [Bit; 12] = [72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 33];
/// 
/// fn main() {
///     let mut keccak = Keccak::new(SecurityLevel::SHA256, StateBitsWidth::F1600);
///     keccak.append(&mut YOUR_INPUT_BYTES);
///     println!("{:?}", keccak.hash());
/// }
/// ```
///
/// [`The Keccak reference`]: https://keccak.team/files/Keccak-reference-3.0.pdf
pub struct Keccak {
    state: State,
    sponge: Sponge,
}

impl Keccak {
    /// Creates a new keccak state with a provided security level and state bits width.
    /// 
    /// Possible securities levels: 
    /// - SHA224 (224 bit)
    /// - SHA256 (256 bit)
    /// - SHA384 (384 bit)
    /// - SHA512 (512 bit)
    /// 
    /// Possible state bits widths:
    /// - f25
    /// - f50
    /// - f100
    /// - f200
    /// - f400
    /// - f800
    /// - f1600
    pub fn new(security: SecurityLevel, width: StateBitsWidth) -> Keccak {
        let security_level = match security {
            SecurityLevel::SHA224 => (RATES[0], CAPACITIES[0]),
            SecurityLevel::SHA256 => (RATES[1], CAPACITIES[1]),
            SecurityLevel::SHA384 => (RATES[2], CAPACITIES[2]),
            SecurityLevel::SHA512 => (RATES[3], CAPACITIES[3]),
            SecurityLevel::Custom(security) => (security.0, security.1)
        };

        Keccak {
            state: [[0; 5]; 5],
            // rate & capacity in bytes
            sponge: Sponge::new(
                security_level.0 / 8,
                security_level.1 / 8,
                width
            ),
        }
    }

    /// Appends input to current state
    pub fn append(&mut self, input: &BitsArr) {
        let padding_total = self.sponge.rate - (input.len() % self.sponge.rate);
        let mut padding: Vec<Bit>;

        if padding_total == 1 {
            padding = vec![0x81];
        } else {
            padding = vec![];
            padding.push(0x01);
            
            for _ in 0..(padding_total - 2) {
                padding.push(0x00);
            }

            padding.push(0x80);
        }
        
        let padded_input: &BitsArr = &[input, &padding].concat();
        self.sponge.absorb(&mut self.state, padded_input);
    }

    /// Returns keccak hash based on current state
    pub fn hash(&mut self) -> BitsVec {
        self.sponge.squeeze(&mut self.state)
    }
}
