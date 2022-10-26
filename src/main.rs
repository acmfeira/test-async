use std::{thread, time::Duration};

mod Utils {
    use std::{thread, time::Duration};

    use futures::executor::block_on;


    pub struct Loop{
        pub temp : u64,
        pub message : &'static str,        
    }
    impl Loop{
        
        pub fn start(self : &Self, callback: fn(&'static str)){
            
            let pool = async {

                let mut cont = 0;

                while cont <= 100 {
    
                    thread::sleep(Duration::from_millis(self.temp));
                    cont += 1;
    
                }
    
                callback(self.message)
            
            };

            block_on(pool);
       
        }
    
    }
}
use Utils::Loop;

fn main() {

    //test with async + callback
    thread::spawn(move ||{

        let x = Loop {temp:110,message:"Loop begin finished!"};
        x.start(|message|{
            
            println!("{}",message);
        
        });

    });
        
    println!("Begin process..");

    //thread with callback
    run_all();

    println!("Wait progress finish!!!");

    loop {
        thread::sleep(Duration::from_secs(1))
    }    
}

fn run_all(){

    println!("Starting 0!!!");
    test(100,"loop 0", |message|{
        println!("Finished {} !!!", message)
    });
    
    println!("Starting 1!!!");

    test(50, "loop 1", |message| {
        println!("Finished {} !!! ", message)
    });
}

fn test(mil : u64, message: &'static str, callback : fn(&'static str)){


    thread::spawn(move ||{

        let mut cont = 0;

        while cont <=100 {
            thread::sleep(Duration::from_millis(mil));
            cont += 1;
        }
        
        callback(message);

    });
}
