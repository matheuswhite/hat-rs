#![feature(proc_macro_quote)]

use proc_macro::{quote, TokenStream};

#[proc_macro_attribute]
pub fn main(_args: TokenStream, item: TokenStream) -> TokenStream {
    let _system_clock = "8_000_000";

    format!("#[no_mangle]
        static TIME_MANAGER: TimeManager = TimeManager::new();

        static CTX: &TimeManager = &TIME_MANAGER;
        static CALLBACK: fn(&'static TimeManager, u128) = TimeManager::timeout;
        static TIMER: Mutex<UnsafeCell<Option<SYST>>> = Mutex::new(UnsafeCell::new(None));
        static NOW: Mutex<UnsafeCell<u128>> = Mutex::new(UnsafeCell::new(0));
        static NEXT_TIMEOUT: Mutex<UnsafeCell<u128>> = Mutex::new(UnsafeCell::new(0));

        const SYSTEM_CLOCK: u128 = 8_000_000;

        const TIME_CONTANT: u128 = 10_000_000;
        const TIME_DIVISOR: u128 = 10_000_000_000;
        const TICKS: u128 = (TIME_CONTANT * SYSTEM_CLOCK) / TIME_DIVISOR;

        pub fn init_timer(mut timer: SYST) {{
            let ticks = TICKS as u32 & 0x00FF_FFFF;
            rprintln!(\"Ticks: {{}}\", ticks);

            timer.set_reload(ticks);
            timer.enable_interrupt();
            timer.enable_counter();

            critical_section::with(|cs| {{
                let g_timer = unsafe {{ &mut *TIMER.borrow(cs).get() }};

                *g_timer = Some(timer);
            }});
        }}

        #[no_mangle]
        pub fn __current_time_ms() -> u128 {{
            critical_section::with(|cs| *unsafe {{ &*NOW.borrow(cs).get() }})
        }}

        #[no_mangle]
        pub fn __start_timer(timeout: u128) {{
            critical_section::with(|cs| {{
                let next_timeout = unsafe {{ &mut *NEXT_TIMEOUT.borrow(cs).get() }};

                *next_timeout = timeout;
            }});
        }}

        #[allow(non_snake_case)]
        #[exception]
        fn SysTick() {{
            let (next_timeout, now) = critical_section::with(|cs| {{
                let timer = unsafe {{ &mut *TIMER.borrow(cs).get() }}.as_mut().unwrap();
                timer.clear_current();

                let now = unsafe {{ &mut *NOW.borrow(cs).get() }};
                *now += 1;

                (unsafe {{ *NEXT_TIMEOUT.borrow(cs).get() }}, *now)
            }});

            if next_timeout == now {{
                rprintln!(\"Heap: {{}}\", HEAP.free());
                CALLBACK(CTX, now);
            }}
        }}

        #[cortex_m_rt::entry]
        fn __entry() -> ! {{
            {{
                use core::mem::MaybeUninit;
                static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
                unsafe {{ HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }}
            }}

            rtt_init_print!();

            rprintln!(\"Spawning main task...\");

            let _ = spawn!(main);

            rprintln!(\"Starting executor...\");
            let executor = critical_section::with(|cs| unsafe {{ &mut *EXECUTOR.borrow(cs).get() }});
            executor.block_on().unwrap();

            loop {{}}
        }}

        {}", item.to_string()).parse::<TokenStream>()
        .unwrap()
}
