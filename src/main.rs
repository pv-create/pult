use rppal::gpio::{Gpio, Pin, Trigger, Level, InputPin};
use std::thread;
use std::time::{Duration, Instant};

const GPIO_LAZER: u8 = 17;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Инициализация GPIO
    let gpio = Gpio::new()?;

    // Подключение IR-принципника к GPIO pin 17
    let mut ir_pin = gpio.get(GPIO_LAZER)?.into_input_pullup();;

    println!("IR receiver initialized");

    loop {
        // Чтение IR-сигнала
        if let Some(code) = read_ir_signal(&mut ir_pin) {
            println!("Received IR code: {}", code);
        }

        thread::sleep(Duration::from_millis(50));
    }
}

// Функция для чтения IR-сигнала
fn read_ir_signal(ir_pin: &mut InputPin) -> Option<u32> {
    let mut signal = vec![0u8; 64];
    let start_time = Instant::now();

    loop {
        match ir_pin.read_value()? {
            true => {
                signal[signal.len() - 1] = 1;
                if signal.iter().all(|&x| x == 1) {
                    break;
                }
            }
            false => {
                signal[signal.len() - 1] = 0;
                if signal.iter().all(|&x| x == 0) {
                    break;
                }
            }
        }

        if start_time.elapsed().as_millis() > 100 {
            return None;
        }
    }

    Some(signal.into_iter().rev().collect::<Vec<u32>>().into_iter().sum())
}
