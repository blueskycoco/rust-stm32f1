#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use core::fmt::Write;
use stm32f1xx_hal::{
    pac,
    dma::Half,
    prelude::*,
    serial::{Config, Serial},
};
use nb::block;
use cortex_m::{asm, singleton};
use unwrap_infallible::UnwrapInfallible;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let channels = dp.DMA1.split();
    let mut afio = dp.AFIO.constrain();
    let mut gpioa = dp.GPIOA.split();
    let mut rst = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);
    let rx_pin = gpioa.pa3;
    let tx_pin = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    let mut serial = Serial::new(
        dp.USART2,
        (tx_pin, rx_pin),
        &mut afio.mapr,
        Config::default().baudrate(115200.bps()),
        &clocks,
    );
    let dbg_rx_pin = gpioa.pa10;
    let dbg_tx_pin = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let dbg = Serial::new(
        dp.USART1,
        (dbg_tx_pin, dbg_rx_pin),
        &mut afio.mapr,
        Config::default().baudrate(115200.bps()),
        &clocks,
    );
    let (mut dbg_tx, _rx) = dbg.split();
    //let (mut tx, mut rx) = serial.split();
    let rx = serial.rx.with_dma(channels.6);
    let tx = serial.tx.with_dma(channels.7);
    //let (_tx, mut rx) = serial.split();
    //let rbuf = singleton!(: [[u8; 8]; 2] = [[0; 8]; 2]).unwrap();
    let mut delay = cp.SYST.delay(&clocks);
    rst.set_low();
    delay.delay_ms(1_0_u16);
    rst.set_high();
    delay.delay_ms(1_200_u16);
    writeln!(dbg_tx, "usr-wifi232-t\r").unwrap();
    let (_, tx) = tx.write(b"+++").wait();
    let rbuf = singleton!(: [u8; 1] = [0; 1]).unwrap();
    let (rbuf, rx) = rx.read(rbuf).wait();
    writeln!(dbg_tx, "read {}\r", core::str::from_utf8(rbuf).unwrap()).unwrap();
    let (_, tx) = tx.write(b"a").wait();
    let rbuf = singleton!(: [u8; 3] = [0; 3]).unwrap();
    let (rbuf, rx) = rx.read(rbuf).wait();
    writeln!(dbg_tx, "read {}\r", core::str::from_utf8(rbuf).unwrap()).unwrap();
    let (_, tx) = tx.write(b"at+h\r").wait();
    let rbuf = singleton!(: [u8; 2048] = [0; 2048]).unwrap();
    let (rbuf, rx) = rx.read(rbuf).wait();
    writeln!(dbg_tx, "read {}\r", core::str::from_utf8(rbuf).unwrap()).unwrap();
    //tx.bwrite_all(b"+++").unwrap();
    //let rbuf = singleton!(: [u8; 1] = [0; 1]).unwrap();

    //let (rbuf, rx) = rx.read(rbuf).wait();
    //let mut circ_buffer = rx.circ_read(rbuf);

    //while circ_buffer.readable_half().unwrap() != Half::First {}

    //let first_half = circ_buffer.peek(|half, _| *half).unwrap();
    //writeln!(dbg_tx, "read {}\r", core::str::from_utf8(rbuf).unwrap()).unwrap();
    //let rbuf = singleton!(: [u8; 3] = [0; 3]).unwrap();
    //let (_, tx) = tx.write(b"a").wait();
    //let (rbuf, rx) = rx.read(rbuf).wait();
    //writeln!(dbg_tx, "read {}\r", core::str::from_utf8(rbuf).unwrap()).unwrap();
    //delay.delay_ms(1_000_u16);
    //let rbuf = singleton!(: [u8; 21] = [0; 21]).unwrap();
    //let (_, _tx) = tx.write(b"at+ver\r").wait();
    //let (rbuf, _rx) = rx.read(rbuf).wait();
    //writeln!(dbg_tx, "read {}\r", core::str::from_utf8(rbuf).unwrap()).unwrap();
   /* 
    let received = block!(rx.read()).unwrap();
    writeln!(dbg_tx, "read {}\r", received).unwrap();
    block!(tx.write(received)).unwrap_infallible();
    
    let received = block!(rx.read()).unwrap();
    writeln!(dbg_tx, "1 {}\r", received).unwrap();
    tx.bwrite_all(b"at+h\r").unwrap();
    let received = block!(rx.read()).unwrap();
    writeln!(dbg_tx, "2 {}\r", received).unwrap();
    */
    loop {
    }
}
