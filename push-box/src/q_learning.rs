#![allow(unused_macros)]
use std::collections::HashMap;

use super::environment::*;
use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

pub struct QLearning {
    action: [Move; 4],
    q_state: HashMap<((u32, u32), (u32, u32)), [f64; 4]>,
    discount_factor: f64,
    alpha: f64,
    epsilon: f64,
    rng: ThreadRng,
}

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

impl QLearning {
    pub fn new() -> QLearning {
        QLearning {
            action: [Move::Right, Move::Left, Move::Down, Move::Up],
            q_state: HashMap::new(),
            discount_factor: 0.99,
            alpha: 0.5,
            epsilon: 0.01,
            rng: rand::thread_rng(),
        }
    }

    pub fn take_action(&mut self, state: ((u32, u32), (u32, u32))) -> Move {
        if self.rng.gen::<f64>() < self.epsilon {
            *self.action.choose(&mut self.rng).unwrap()
        } else {
            let action_index = self
                .q_state
                .entry(state)
                .or_insert([0.0; 4])
                .iter()
                .enumerate()
                .fold((0, -1e9), |(idx_max, val_max), (idx, val)| {
                    if &val_max > val {
                        (idx_max, val_max)
                    } else {
                        (idx, *val)
                    }
                });
            self.action[action_index.0]
        }
    }

    pub fn train(
        &mut self,
        action: Move,
        state: ((u32, u32), (u32, u32)),
        next_state: ((u32, u32), (u32, u32)),
        reward: f64,
    ) {
        let action = action as usize;
        let q = self.q_state.entry(state).or_insert([0.0; 4])[action];
        self.q_state.get_mut(&state).unwrap()[action] = q + self.alpha
            * (reward
                + self.discount_factor
                    * self
                        .q_state
                        .entry(next_state)
                        .or_insert([0.0; 4])
                        .iter()
                        .max_by(|a, b| a.total_cmp(b))
                        .unwrap()
                - q);
        //log!("{:?} {}", self.q_state[state], action);
    }
}
