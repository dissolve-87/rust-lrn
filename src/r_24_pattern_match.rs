#[allow(dead_code)]

fn match_process(val:u8) -> String{

    match val {
        0                    => String::from("no apple"),
        1 | 2                => String::from("one or two"),
        3..=5                => String::from("Min amount of apple"),
        tm @ 6..=8        => format!("{}th apple",tm),
        pas if(val == 10) => format!("{}th Base apple",pas),
        _                     => String::from("Empty")
    }
}

pub fn pattern_lrn(){
    // for tmp in 0..11 {
    //     println!("[{}] : i have {}",tmp,match_process(tmp));
    // }
    
    // let contin =(4,0);  //x
    // let contin =(0,10);        //y
    let contin =(1,5);         //x and y
    // let contin =(0,0);         // 0,0
    

    match contin {
        (0,0)            => println!("None"),
        (0,y) => println!("only y is = {}",y),
        (x,0) => println!("only x is = {}",x),
        // (_,y) => println!("Nothing, y = {}",y),  //x and y
        (x,y) => println!("x = {}, y = {}",x,y),

        // (ref mut x,0) => println!("x = {}, y = {}",x) //mark Note
        // enum::value{Name:2,..}   ---> only care about that element , others not consider ",.." 2 dot
        
        
        
    }
    
}