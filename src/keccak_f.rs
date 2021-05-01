use crunchy::unroll;

use super::*;

type WordLen = usize;
type Log = usize;

pub struct KeccakF {
    perm_num: PermutationsNum
}

impl KeccakF {
    pub fn new(width: StateBitsWidth) -> KeccakF {
        let perm_num = match width {
            StateBitsWidth::F25 => PERMUTATIONS_NUM[0],
            StateBitsWidth::F50 => PERMUTATIONS_NUM[1],
            StateBitsWidth::F100 => PERMUTATIONS_NUM[2],
            StateBitsWidth::F200 => PERMUTATIONS_NUM[3],
            StateBitsWidth::F400 => PERMUTATIONS_NUM[4],
            StateBitsWidth::F800 => PERMUTATIONS_NUM[5],
            StateBitsWidth::F1600 => PERMUTATIONS_NUM[6],
            StateBitsWidth::Custom(width) => {
                let wl: WordLen = width / 25;
                let l: Log = (wl as f32).log2() as Log;
                12 + 2 * l
            },
        };
        KeccakF {
            perm_num: perm_num
        }
    }

    pub fn permutations(&self, mut a: &mut State) {
        assert!(
            self.perm_num < 25,
            "Wrong permutations number"
        );
        
        for i in 0..self.perm_num {
            KeccakF::round_b(&mut a, ROUND_CONSTANTS[i]);
        }
    }

    #[inline]
    fn round_b(mut a: &mut State, rc: u64) {
        KeccakF::theta(&mut a);
        let mut temp = KeccakF::rho_pi(&mut a);
        KeccakF::hi(&mut a, &mut temp);
        KeccakF::iota(&mut a, rc);
    }

    #[inline]
    fn theta(a: &mut State) {
        let mut c: [u64; 5] = [0; 5];
        let mut d: [u64; 5] = [0; 5];
    
        unroll! {
            for i in 0..5 {
                c[i] = a[i][0]
                    ^ a[i][1]
                    ^ a[i][2]
                    ^ a[i][3]
                    ^ a[i][4];
            }
        }
        
        unroll! {
            for i in 0..5 {
                d[i] = c[(i + 4) % 5] ^ c[(i + 1) % 5].rotate_left(1);
            }
        }
    
        unroll! {
            for i in 0..5 {
                unroll! {
                    for j in 0..5 {
                        a[i][j] ^= d[i];
                    }
                }
            }
        }
    }

    #[inline]
    fn rho_pi(a: &mut State) -> State {
        let mut temp: State = [[0; 5]; 5];
        unroll! {
            for i in 0..5 {
                unroll! {
                    for j in 0..5 {
                        temp[1 * j][(2 * i + 3 * j) % 5] = a[i][j].rotate_left(ROTATION_CONSTANTS[i][j] as u32);
                    }
                }
            }
        }
        temp
    }

    #[inline]
    fn hi(a: &mut State, temp: &mut State) {
        unroll! {
            for i in 0..5 {
                unroll! {
                    for j in 0..5 {
                        a[i][j] = temp[i][j] ^ (!temp[(i + 1) % 5][j] & temp[(i + 2) % 5][j]);
                    }
                }
            }
        }
    }

    #[inline]
    fn iota(a: &mut State, rc: u64) {
        a[0][0] ^= rc;
    }
}
