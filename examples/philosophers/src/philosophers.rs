use crate::__current_time_ms;
use hat::prelude::{delay_ms, AsyncMutex};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use rtt_target::rprintln;

#[derive(Default)]
pub struct Chopstick;

#[derive(Default)]
pub struct Noodles;

pub async fn philosopher(
    index: usize,
    noodle: &AsyncMutex<Noodles>,
    chopsticks: &[AsyncMutex<Chopstick>],
) {
    let range = 1_000..=5_000;
    let mut rng = SmallRng::seed_from_u64(__current_time_ms() as u64);

    loop {
        {
            let (_left_chopstick, _right_chopstick) = {
                let delay = rng.gen_range(range.clone());
                rprintln!("Philosopher {} is thinking for {}ms...", index, delay);
                delay_ms(delay).await;

                let _noodle_guard = noodle.lock().await;

                let left_chopstick = index;
                let right_chopstick = (index + 1) % chopsticks.len();

                (
                    chopsticks[left_chopstick].lock().await,
                    chopsticks[right_chopstick].lock().await,
                )
            };

            let delay = rng.gen_range(range.clone());
            rprintln!("Philosopher {} is eating for {}ms...", index, delay);
            delay_ms(delay).await;
        }
    }
}
