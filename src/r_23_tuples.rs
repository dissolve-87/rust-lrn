#[allow(dead_code)]

fn calcu(a:u32, b:u32) -> (u32,u32){

    // (adding, multiply)
        (a+b, a*b)
}

pub fn tuple_lrn(){

    let ans = calcu(3,4);
    // Answer : (7, 12)
    println!("Answer : {:?}",ans);

    // destructing tuple
    let (x,y) =ans;
    println!("x = {}, y = {}",x,y);

    // Accessing elements
    println!("{1} {0} ",ans.0,ans.1);
            // Alter index 
    
    // Multi #type value
    let types:(u8,bool,f32) = (22,true,4.5);
    println!("Diff type : {:?}",types);

    let inner_tup = ((22,44), (100,200));
    println!("Assess last element ='200' : {} ", (inner_tup.1).1);

    // d
}