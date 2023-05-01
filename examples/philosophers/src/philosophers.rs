use crate::__current_time_ms;
use alloc::borrow::ToOwned;
use alloc::format;
use hat::prelude::{delay_ms, AsyncMutex};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use rtt_target::rprintln;

#[derive(Default)]
pub struct Chopstick;

pub async fn philosopher(index: usize, chopsticks: &[AsyncMutex<Chopstick>]) {
    let range = 1_000..=5_000;
    let mut rng = SmallRng::seed_from_u64(__current_time_ms() as u64);

    let (left_chopstick, right_chopstick) = if index == (chopsticks.len() - 1) {
        (0, index)
    } else {
        (index, index + 1)
    };

    loop {
        PhilosopherState::Starving.print(index);
        let left_ch = chopsticks[left_chopstick].lock().await;
        PhilosopherState::HoldingOneChopstick.print(index);
        let right_ch = chopsticks[right_chopstick].lock().await;

        let delay = rng.gen_range(range.clone());
        PhilosopherState::Eating(delay).print(index);
        delay_ms(delay).await;

        right_ch.unlock();
        PhilosopherState::DroppedOneChopstick.print(index);
        left_ch.unlock();

        let delay = rng.gen_range(range.clone());
        PhilosopherState::Thinking(delay).print(index);
        delay_ms(delay).await;
    }
}

enum PhilosopherState {
    Starving,
    HoldingOneChopstick,
    Eating(u64),
    DroppedOneChopstick,
    Thinking(u64),
}

impl PhilosopherState {
    fn print(&self, index: usize) {
        let state_str = match self {
            PhilosopherState::Starving => "STARVING".to_owned(),
            PhilosopherState::HoldingOneChopstick => "HOLDING ONE CHOPSTICK".to_owned(),
            PhilosopherState::Eating(delay) => format!("EATING [ {} ms ]", delay),
            PhilosopherState::DroppedOneChopstick => "DROPPED ONE CHOPSTICK".to_owned(),
            PhilosopherState::Thinking(delay) => format!("THINKING [ {} ms ]", delay),
        };
        rprintln!("Philosopher {} - {}", index, state_str);
    }
}
