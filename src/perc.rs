use crate::sequence::{control_change, param_value, CC_LAYER, SP2, SP3, SP4};
use crate::trig::Trig;
use crate::PERC_CHANNEL;
use crate::{cc_parameter, log_send};
use midir::MidiOutputConnection;
use rand::rngs::ThreadRng;
use rand::Rng;

const SP_ARRAY: [u8; 3] = [SP2, SP3, SP4];
const NB_TRIGS: usize = 16;
const NB_LAYERS: [usize; 3] = [2, 2, 2];
const LAYER: [u8; 3] = [0, 1 << 6, 0x60];

#[derive(Copy, Clone, Default)]
pub struct Rythm {
    trigs: [bool; NB_TRIGS],
}

#[derive(Clone)]
pub struct Perc {
    cur_pattern_id: usize,
    patterns: Vec<[Rythm; 3]>,
    is_active: bool,
}

impl Perc {
    pub fn get_trigs(&mut self, step: u32) -> Vec<Trig> {
        if self.is_active && step % 6 == 0 {
            let pattern = &self.patterns[self.cur_pattern_id];
            let t = step / 6;
            let t = t as usize % NB_TRIGS;
            return pattern
                .iter()
                .enumerate()
                .filter_map(|(i, p)| {
                    if p.trigs[t] {
                        Some(Trig {
                            start_end: true,
                            channel_id: PERC_CHANNEL,
                            note: SP_ARRAY[i],
                            velocity: param_value(0.0),
                        })
                    } else {
                        None
                    }
                })
                .collect();
        }

        vec![]
    }

    pub fn toggle(&mut self, conn: &mut MidiOutputConnection, rng: &mut ThreadRng) {
        self.is_active = !self.is_active;
        if self.is_active {
            self.cur_pattern_id = rng.gen_range(0..self.patterns.len());
            for i in 0..3 {
                log_send(
                    conn,
                    &control_change(
                        PERC_CHANNEL,
                        cc_parameter(CC_LAYER, (i + 1) as u8),
                        LAYER[rng.gen_range(0..NB_LAYERS[i])],
                    ),
                );
            }
        }
    }

    pub fn new(patterns: Vec<[Rythm; 3]>) -> Self {
        let len = patterns.len();
        if len == 0 {
            panic!("len of patterns is 0");
        }
        Self {
            cur_pattern_id: 0,
            patterns,
            is_active: false,
        }
    }

    pub fn on(&self) -> bool {
        self.is_active
    }
}

impl Rythm {
    pub fn compute_euclidean_rythm(_k: u8) -> Self {
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

        Self { trigs: seq }
    }
}
