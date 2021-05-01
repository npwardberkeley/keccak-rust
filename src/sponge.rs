use super::*;

pub struct Sponge {
    pub rate: Rate,
    capacity: Capacity,
    keccak_f: KeccakF,
}

impl Sponge {
    pub fn new(rate: Rate, capacity: Capacity, width: StateBitsWidth) -> Sponge {
        Sponge {
            rate: rate,
            capacity: capacity,
            keccak_f: KeccakF::new(width),
        }
    }

    pub fn absorb(&self, mut state: &mut State, message: &BytesArr) {
        assert!(
            message.len() % self.rate == 0,
            "Message is not divisible entirely by bytes rate"
        );

        let chunks_total = message.len() / self.rate;

        let words: Vec<u64> = Sponge::bits_to_u64_words_le(message);

        for chunk_i in 0..chunks_total {
            let chunk_offset: usize = chunk_i * (self.rate / 8);
            let mut x = 0;
            let mut y = 0;
            for i in 0..(self.rate / 8) {
                let word = words[chunk_offset + i];
                state[x][y] ^= word;
                if x < 5 - 1 {
                    x += 1;
                } else {
                    y += 1;
                    x = 0;
                }
            }
            self.keccak_f.permutations(&mut state);
        }
    }

    pub fn squeeze(&self, state: &mut State) -> BytesVec {
        let mut output: Vec<u8> = vec![];

        let output_len: usize = self.capacity / 2;
        let elems_total: usize = output_len / 8;
        let mut counter: usize = 0;

        'outer: for i in 0..5 {
            for j in 0..5 {
                output.append(&mut state[j][i].to_le_bytes().to_vec());
                if counter == elems_total {
                    break 'outer;
                }
                counter += 1;
            }
        }

        output.resize(output_len, 0);
        output
    }

    fn bits_to_u64_words_le(message: &BytesArr) -> Vec<u64> {
        let words_total = message.len() / 8;
        let mut words: Vec<u64> = vec![0; words_total];

        for i in 0..words_total {
            let mut word_bits: [u8; 8] = Default::default();
            word_bits.copy_from_slice(&message[i * 8..i * 8 + 8]);
            words[i] = u64::from_le_bytes(word_bits);
        }
        words
    }
}
