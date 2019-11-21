use crate::consumer;

#[derive(Debug)]
pub  struct Producer {
   pub id: String,
}


impl Producer {

    pub fn pr(&self) {
        println!("{:#?}", self);
    }

   pub   fn call_other_mod() {

        let p = consumer::cons::Consumer{id: "I am consumer from producer".to_string()};
        p.pr();
}
}
