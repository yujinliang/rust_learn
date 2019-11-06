use std::time::Duration;
use std::error::Error;
use crossbeam_channel::{bounded, tick, Receiver, select};

fn ctrl_channel() ->Result<Receiver<()>, ctrlc::Error> {

    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {

        let _ = sender.send(());

    })?;

    Ok(receiver)
}
fn main() -> Result<(), Box<dyn Error>> {

    let ctrl_c_events = ctrl_channel()?;
    let ticks = tick(Duration::from_secs(1));

    loop {

                select!{

                    recv(ticks) -> _ => {
                        println!("working!");
                    }

                    recv(ctrl_c_events)-> _ => {
                        println!("Got the CtrlC Signal");
                        println!("goodbye!");
                        break;
                    }
                }
    }

    Ok(())
}
