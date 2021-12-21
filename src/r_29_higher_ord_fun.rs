#[allow(dead_code)]

fn is_even(x:u32) ->bool{
    // 2 , 4 , 6 , 8 , 10, 12, 14
    x % 2 == 0
    // return true
}
pub fn hi_ord(){

    const LIMIT:u32 =50;
 
    let mut total = 0;

    //      infinie loop
    for tmp in 0.. {
                    //   7 * 7 = 46
        let check = tmp*tmp;

        //   46  > 50  next iteration loop break
        if check > LIMIT { break; }

                //      true
        else if is_even(check) 
            { 
            // increament value 
                total += check;
                // append value
                // 2 + 4 + 6 + 8 + 10 + 12 + 14 = 56
            }
    }
    println!("Sum : {}",total);
    //        Sum : 56
    // ------------------------------------------------------------------------------

    // let mut tot = 0;
    let answer = 
        (0..).map( |x| x * x)                   // iterate element in range of 0 to infinite
             .take_while( |&x| x <= LIMIT )     // while loop  | x <= 50
             .filter( |x| is_even(*x) )         // filter x % 2 == 0
             .fold(0,|tot,x| tot + x)           // sum | 0 is initial | x val added to tot variable
    ;

    println!("higher fn  sum : {}", answer);

}