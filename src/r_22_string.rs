#[allow(dead_code)]

pub fn string_lrn(){
    // &str = string slice
    // statically allocated
    let name:&'static str = "Bottle";
    // cannot access slice name[1]...  not allowed
    // bytes | utf-8
    // NOTHING to original string (.chars(), .to_string())...


    // reverse printing
    for tmp in name.chars().rev(){
        println!("{}",tmp)
    }

    // -----------------------------------------------------------------

    // get first char of name
    // char is Option type

    if let Some(first_char) = name.chars().nth(0) {
        println!("first char is \"{}\" ",first_char);
    }
    // -----------------------------------------------------------------

    // #String type (flexable string)
    // heap allocalated

    let mut creats = String::new();
    let mut a = 'a' as u8;

    while a <= ('z' as u8){
        creats.push(a as char);
        // creats.push(',');    //  ' '
        creats.push_str(",");   //  " "
        a+=1;
    }
    println!("alfa : {:?}",creats);

    let fix_name:&str = &creats;
    println!("to str : {}",fix_name);
    //-------------------------------------------------------------------
    
    // concate
    // String + &str

    let strong_str:&str = "hello";
    let flexable_str = String::from(", World");
    let con = flexable_str + &strong_str;
    // concate only String + &str
    println!("concate both type of strings : {}",con);

    // .to_string(), remove(0),replace("..","...")...
    // refer string method
}

// 2 type of strings