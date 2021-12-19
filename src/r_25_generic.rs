#[allow(dead_code)]


// dynamic type data T,U
struct Points<T,U> {
    x: T,
    y: U
}

// struct Points<T> {
//     x: T,
//     y: T
// }
// struct Line<T> {
//     start:Points<T>,
//     end:Points<T>
// }

pub fn generic_lrn(){

    let line:Points<u8, u8> = Points {x: 44, y: 10 };
    let draw = Points{x:55,y:77};

    println!("x={}, y={}",line.x,line.y);
    println!("x={}, y={}",draw.x,draw.y);
    // --------------------------------------------------------

        // let draw1  = Points{x:5,y:6};
        // let draw2  = Points{x:2,y:4};
        // let arts = Line{start:draw1,end:draw2};


        
}