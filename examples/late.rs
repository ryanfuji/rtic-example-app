//! examples/late.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_semihosting::{debug, hprintln};
use heapless::{
    consts, i,
    spsc::{Consumer, Producer, Queue},
};
use lm3s6965::Interrupt;
use panic_semihosting as _;

#[rtic::app(device = lm3s6965)]
const APP: () = {
    // Late resources
    struct Resources {
        p: Producer<'static, u32, consts::U4>,
        c: Consumer<'static, u32, consts::U4>,
    }

    #[init]
    fn init(_: init::Context) -> init::LateResources {
        static mut Q: Queue<u32, consts::U4> = Queue(i::Queue::new());

        let (p, c) = Q.split();

        // Initialization of Late Resources
        init::LateResources { p, c }
    }

    #[idle(resources = [c])]
    fn idle(cx: idle::Context) -> ! {
        loop {
            if let Some(byte) = cx.resources.c.dequeue() {
                hprintln!("received message: {}", byte).unwrap();

                debug::exit(debug::EXIT_SUCCESS);
            } else {
                rtic::pend(Interrupt::UART0);
            }
        }
    }

    #[task(binds = UART0, resources = [p])]
    fn uart0(cx: uart0::Context) {
        cx.resources.p.enqueue(42).unwrap();
    }
};
