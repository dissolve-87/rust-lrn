#[allow(dead_code)]

// Array are fixed sixe
pub fn array_lrn(){
    //      [type, how many]
    let mut arr:[u8;5] = [1,2,3,4,5];
    // pretty print
    println!("{:#?}", arr);

    arr[0] = 100; // change value 1-> 100
    println!("Len = {}, index of 0 = {}",arr.len(),arr[0]);

    // check | compare elements in array
    
    if arr == [100,2,3,4,5] {   //true , == , !=   # lenth must be same for compare
        println!("True");
    }

    // bulk fill array
    let bulk_one = [1u32;10];
    // let bulk_one:[u8;10] = [1;10]; // another way
    println!("{:?}",bulk_one);
}