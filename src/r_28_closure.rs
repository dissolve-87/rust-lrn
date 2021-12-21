#[allow(dead_code)]

// lambda | Anonymous functions
pub fn closure_lrn() {

    // model-1 (String)
    let message = |name: String|->String {format!("hi {} how are you.!",name)};
    println!("{}",message(String::from("Narendran")));

    // model-2 (Number)
    let numbers = |num:u8| -> u8{ num + 100};
    println!("Added number is : {}",numbers(5));

    // ---------------------------------------------------------------------------------------------------

    // model-3 (Notning)
    let mut global_nu = 5;
    
    let changer = |x| {
        let mut chg = x;
        chg += global_nu; //8
        chg
        };
    println!("change the number : {}", changer(3));
    
    let new_num = &mut global_nu;
    println!("New number is : {}", new_num);

    // --------------------------------------------------------------------------
    // change variable value useing reference
    
    // Pass bye reference
    let mut num:i32 = 0;
                                        // mutable reference 
    let change_nm_ref = |x :&mut i32 | {
        // Access reference and change val 
        *x = 100
        };
                    // pass mutable refernce of num
    change_nm_ref( &mut num);

    println!("Change value by reference 0 -> : {}",num);
           // Change value by reference 0 -> : 100

    // ----------------------------------------------------------------------------
    // Pass bye value ( Like )

    let num_2= 0;
    // mutable value 
    let change_nm_val = |mut x | {
    // inside x(num_2) local variable, change val 
        x += 100;
        println!("---------> Inside pass bye value , variable num_2 = {}",x); // num_2 = 100
    };
    // pass value num
    change_nm_val(num_2);

    println!("Pass by value of num = 0 -> : {}",num_2);
    // Pass by value of num = 0 -> : 0



}