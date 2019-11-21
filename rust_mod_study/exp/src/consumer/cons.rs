use crate::producer;
use crate::switcher;

#[derive(Debug)]
pub  struct Consumer {
    pub id: String,
}


impl Consumer {

    pub fn pr(&self) {
        println!("{:#?}", self);
    }

    pub fn call_other_mod() {

            let p = producer::Producer{id: "I am  producer from consumer".to_string()};
            p.pr();
            switcher::sw::exchange();
    }
}
