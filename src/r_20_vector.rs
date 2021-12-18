#[allow(dead_code)]

pub fn vec_lrn(){
    let mut note = Vec::new();
    note.push(1);
    note.push(2);
    note.push(3);
    note.push(4);
    note.push(5);
    note.push(6);
    note.push(7);
    note.push(8);
    note.push(9);
    println!("{:?}",note);

    // # indexs starting only positive numbers = usize
    let index:usize =1; //note[1] answ = 2
    
    println!("index of {} is {}", index, note[index]);

    // Handle Index out of bound
    // try to access 10 element

            // get is <Option> type 
            // like expection handeling
    match note.get(100) {
        Some(val) => println!("is in range {}",val),
        None => println!("out of range")
    }

    for tm in &note{
        println!("[+] {}",tm);
    }
    // pop is an Option type
    // if vec poped all element----> got empty iteration(None), thing handle bye "Option"

    
    let po = note.pop();
    // let Some(po) = note.pop(); if late iteration empty , it become None, 
    //      ^^^^^^^^ pattern `None` not covered


    // pop the element and store into val
    // if some become None it break
    // reverse print
    while let Some(val) = note.pop() {
        println!("--> {}",val);
    } 
    println!("after poped : {:?}",po);


}


