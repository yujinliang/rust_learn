extern crate crossbeam_channel as channel;
extern crate signal_hook;

use std::{thread,os::raw::c_int, error::Error};

fn notify(signals: &[c_int]) -> Result<channel::Receiver<c_int>, Box<dyn Error> > {

    let (s, r) = channel::bounded(100);
    let signals = signal_hook::iterator::Signals::new(signals)?;
    thread::spawn(move || {

        for signal in signals.forever() {
            s.send(signal).unwrap();
        }
    });
    Ok(r)
}

fn main() {

    let r = notify(&[signal_hook::SIGINT]).unwrap();

    println!("catch the signal : {:?}", r.recv().unwrap());

}
