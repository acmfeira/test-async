use std::{thread, time::Duration};

use futures::{executor::block_on, FutureExt};

fn main() {

    println!("Begin process..");

    runa_all();

    println!("Wait progress finish!!!");

    loop {
        thread::sleep(Duration::from_secs(1))
    }    
}
fn runa_all(){

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
