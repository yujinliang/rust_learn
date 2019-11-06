use std::io::Error;
use signal_hook::iterator::Signals;
use tokio::prelude::*;


fn main() -> Result<(), Error> {

    let wait_signal = Signals::new(&[signal_hook::SIGUSR1])?
                                        .into_async()?
                                        .into_future()
                                        .map(|sig|{println!("game over: {}", sig.0.unwrap());})
                                        .map_err(|e| panic!("{}", e.0));

        unsafe{ libc::kill(libc::getpid(), signal_hook::SIGUSR1);};
        tokio::run(wait_signal);
        Ok(())
}
