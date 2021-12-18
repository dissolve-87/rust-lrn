#[allow(dead_code)]

fn silce_process(val : &mut[i32]) {
    val[0] = 22;
    val[1] = 333;
    println!("{:?}",val);
}

pub fn silces_lrn(){

    let mut items = [1,2,3,4,5,6,];

    println!("{:?}",&items[1..4]); // normal silce

    // silce_process(&mut items[1..4]); //silce

    // silce_process(&mut items);
    // passing mutable reference 
    // println!("{:?}",items);
}

// -------NOTE------

// println!("{:?}",items[1..4]);
// |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time