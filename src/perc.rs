use crate::log_send;
use crate::sequence::{param_value, start_note, SP2, SP3, SP4};
use midir::MidiOutputConnection;
use rand::rngs::ThreadRng;
use rand::Rng;

const SP_ARRAY: [u8; 3] = [SP2, SP3, SP4];
const NB_TRIGS: usize = 16;

#[derive(Copy, Clone, Default)]
struct Rythm {
    trigs: [bool; NB_TRIGS],
    k: u8,
}

#[derive(Clone)]
pub struct Perc {
    cur_pattern_id: usize,
    patterns: Vec<[Rythm; 3]>,
}

impl Perc {
    fn run(&self, step: u32, conn: &mut MidiOutputConnection, channel_id: u8) {
        if step % 6 == 0 {
            let pattern = &self.patterns[self.cur_pattern_id];
            let t = step / 6;
            let t = t as usize % NB_TRIGS;
            for (i, p) in pattern.iter().enumerate() {
                if p.trigs[t] {
                    log_send(conn, &start_note(channel_id, SP_ARRAY[i], param_value(0.0)));
                }
            }
        }
    }

    fn new_pattern(&mut self, rng: &mut ThreadRng) {
        self.cur_pattern_id = rng.gen_range(0..self.patterns.len());
    }

    fn new(patterns: Vec<[Rythm; 3]>) -> Self {
        let len = patterns.len();
        if len == 0 {
            panic!("len of patterns is 0");
        }
        Self {
            cur_pattern_id: 0,
            patterns,
        }
    }
}

impl Rythm {
    fn compute_euclidean_rythm(_k: u8) -> Self {
        let (k, compl) = if _k > NB_TRIGS as u8 / 2 {
            (NB_TRIGS as u8 - _k, true)
        } else {
            (_k, false)
        };

        let mut mat: [[bool; NB_TRIGS]; NB_TRIGS] = [[false; NB_TRIGS]; NB_TRIGS];

        // Initialize the Matrix & len
        for i in 0..k as usize {
            mat[i][0] = true;
        }

        let mut mat_len: [usize; NB_TRIGS] = [0; NB_TRIGS];
        mat_len[0] = NB_TRIGS;

        // Compute the rythm
        let mut a = NB_TRIGS as u8;
        let mut b = k;
        let mut last_line = 1;

        while b > 0 {
            loop {
                // Move the most right b columns
                let len = mat_len[0];
                let mut moved_elems: [usize; NB_TRIGS] = [0; NB_TRIGS];
                let mut new_last_line = last_line;

                for i in 0..b as usize {
                    let col = len - b as usize + i as usize;
                    for j in 0..last_line {
                        if mat_len[j] > col {
                            mat[i][last_line + j] = mat[col][j];
                            moved_elems[j] += 1;
                            mat_len[last_line + j] += 1;
                            if last_line + j + 1 > new_last_line {
                                new_last_line = last_line + j + 1;
                            }
                        } else {
                            break;
                        }
                    }
                }

                for i in 0..last_line {
                    mat_len[i] -= moved_elems[i];
                }
                last_line = new_last_line;

                let max_len = mat_len[0];
                let mut second_len = 0;
                for i in 0..last_line {
                    if mat_len[i] < max_len {
                        second_len = mat_len[i];
                        break;
                    }
                }
                /*
                //DEBUG
                println!("[{} {}]", a, b);
                for i in 0..last_line {
                for j in 0..mat_len[i] {
                print!(
                "{} ",
                match mat[j][i] {
                true => 1,
                false => 0,
                }
                );
                }
                println!("");
                }
                */

                if second_len + b as usize > max_len || max_len == b as usize {
                    break;
                }
            }

            let r = a % b;
            a = b;
            b = r;
        }

        let nb_col = mat_len[0];

        let mut seq: [bool; NB_TRIGS] = [false; NB_TRIGS];

        let mut iter = 0;

        for i in 0..nb_col {
            let mut j = 0;
            while j < NB_TRIGS && mat_len[j] > i {
                seq[iter] = compl ^ mat[i][j];
                iter += 1;
                j += 1;
            }
        }

        Self { trigs: seq, k: _k }
    }
}
