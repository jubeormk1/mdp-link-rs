#![no_main]
#![no_std]

#![allow(unused_imports)]
#![allow(dead_code)]

use cortex_m_rt::entry;

#[allow(unused_imports)]
//use panic_halt;
use panic_semihosting as _;

use cortex_m_semihosting::{dbg, hprintln, heprintln};

use core::fmt::Write;
use nb::block;

use embedded_hal::timer::CountDown;

use nrf52840_hal as hal;
use hal::timer::{TimerExt, Timer};
use hal::clocks::{ClocksExt, Clocks};
use hal::{Uarte, target::UARTE0};

use nrf52840_mdk::Leds;

use nrf52_radio::Radio;
use nrf52_radio::radio::RadioExt;
use nrf52_radio::tx_power::TxPower;
use nrf52_radio::mode::Mode;
use nrf52_radio::frequency::Frequency;
use nrf52_radio::logical_address::LogicalAddress;
use nrf52_radio::rx_addresses::{RX_ADDRESS_1, RX_ADDRESS_ALL};
use nrf52_radio::base_address::BaseAddresses;

use nrf52_esb::{Esb, RxConfig, TxConfig, RxPacket};
use nrf52_esb::protocol::Protocol as EsbProtocol;
use nrf52840_mdk::{leds_welcome, Board};

const LED_INTERVAL: u32 = 1_000_000; // us

// P905 responds the request from M01
const PACKET_PAYLOAD: [u8; 34] = [51, 0,
  0x09, 0x0d, 0x62, 0x6d, 0xfa, 0x5d, 0x00, 0x00,
  0x3e, 0xc2, 0x3b, 0x00, 0x0f, 0x78, 0x6d, 0xf9,
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,];

const TARGET_CHANNEL: u8 = 78;

#[entry]
fn main() -> ! {

    let mut board = Board::take().unwrap();

    _ = board.uart_daplink.write_str("\n\n\rSniffer target: Initialising ...\n\r");
    
    let mut timer = board.TIMER0.constrain();
    
    _ = board.uart_daplink.write_str("Sniffer target: LED welcome\n\r");
    leds_welcome(&mut board.leds, &mut timer);
    
    // let clocks = board.CLOCK.constrain().enable_ext_hfosc();
    
    // _ = board.uart_daplink.write_str("Sniffer target: Initialising radio\n\r");
    // let radio = Radio::new(board.RADIO, &clocks);
    // radio
    // .set_tx_power(TxPower::Pos8dBm)
    // .set_mode(Mode::Nrf2Mbit) // All points that most HID devices use this rate
    // .set_frequency(Frequency::from_2400mhz_channel(TARGET_CHANNEL))
    // // .set_base_addresses(BaseAddresses::from_same_four_bytes([0xa0, 0xb1, 0xc2, 0xd3]))
    // // .set_prefixes([0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7])
    // // .set_rx_addresses(RX_ADDRESS_1)
    // .enable_power();

    // let mut read_buffer = [0x00u8; 34];
    // let mut write_buffer = [0x00u8; 34];

    // _ = board.uart_daplink.write_str("Sniffer target: Initialising ESB\n\r");
    // // TODO EsbProtocol and buffers size must match
    // let mut esb = Esb::new(radio, EsbProtocol::fixed_payload_length(read_buffer.len() as u8), &mut read_buffer, &mut write_buffer);
    // esb.set_crc_16bits();
    
    // _ = board.uart_daplink.write_str("Sniffer target: Setting Tx Config to default\n\r");
    // let tx_config = TxConfig::default();

    // _ = board.uart_daplink.write_fmt(format_args!("Sniffer target: Starting main loop. \
    //                                                 \n\rWill send a PAIRING_RESPONSE packet in channel {} approximately every {}us\n\r",TARGET_CHANNEL,LED_INTERVAL));

    board.leds.green.off();
    board.leds.blue.off();
    board.leds.red.off();

    let mut tick: bool = false;

    timer.start(LED_INTERVAL);
    loop {
        // let buf = esb.get_tx_buffer();
        // buf.copy_from_slice(&PAIRING_RESPONSE);

        // if let Err(error) = esb.start_tx(tx_config){
        //     board.leds.red.on();
        //     _ = board.uart_daplink.write_fmt(format_args!("Error: {:?}\n", error));
        // }else{
        //     board.leds.red.off();
        // }
        let loading = if tick {
            "0_0"
        }else {
            "O_O"
        };
        tick = !tick;
        _ = board.uart_daplink.write_fmt(format_args!("\r{:}\r",loading));
        if let Ok(()) = timer.wait() {
            board.leds.green.invert();    
            timer.start(LED_INTERVAL);
        }
    }
}
