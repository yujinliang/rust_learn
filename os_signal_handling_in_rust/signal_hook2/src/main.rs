use signal_hook::iterator::Signals;
use std::{thread, error::Error};

fn main() -> Result<(), Box<dyn Error>> {

    let signals = Signals::new(&[signal_hook::SIGINT, signal_hook::SIGUSR1])?;

    thread::spawn(move || {

        for sig in &signals {

            match sig {

                signal_hook::SIGINT => {

                    println!("SIGINT ->: {:?}", sig);
                    break;

                },
                signal_hook::SIGUSR1 => {
                    println!("SIGUSR1 -> : {:?}", sig);
                    break;

                },
                _ => unreachable!(),
            }
        }
    }).join().unwrap();

    Ok(())
}
