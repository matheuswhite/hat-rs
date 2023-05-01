# Changelog

## v0.1.0

- Futures provided by framework
  - `delay_ms`: Function to delay milliseconds asynchronous;
  - `AsyncMutex`: Mutual exclusion wrapper type to guarantee unique usage through async tasks;
- Minimum heap size tested: `1 KB`
- Architectures supported: `Cortex-M`
- Timer used for delays: `SysTick`
  - The minimum delay tested is `1ms`
- Configuration
  ```rust
  // Heap size of 1024 bytes
  hat::main(1024)
  async fn main() {
  	  // Some HAL init code ...
  
  	  // Use SYST as delay timer and set SYST clock to 16 MHz
      init_timer(SYST, 16_000_000);
  }
  ```
  - Add `hat::main(<HEAP_SIZE>)` at main task, where `<HEAP_SIZE>` is the heap size of firmware;
  - Add `init_timer(<SYST>, <SYST_CLK>);` inside main task, where `<SYST>` is the SysTick object and `<SYST_CLK>` is the AHB configured clock;
  - `init_timer` function must be called before any delay.
- Macro to spawn tasks, at any place
  - Must include `hat::prelude::*` where `spawn!` macro is invocated;
  - `let spawn_result = spawn!(<TASK>)`: Spawn a task to executor, where `<TASK>` is the name of task async function;
  - `let spawn_result = spawn!(<TASK_NAME> => <TASK_CALL>)`: Spawn a task to executor, where `<TASK_NAME>` is the name of the task (could be a literal string or a variable) and `<TASK_CALL>` is the task async function call (In this call, could be passed arguments to the task async function). 
- Task code must be static
- Each task must have a unique name among the spawned tasks
  - If the name is already in use, a `Err` will be returned by the spawn macro
