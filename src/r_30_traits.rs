#[allow(dead_code)]

pub fn tarits_lrn(){

    struct Human{
        name : &'static str
    }
    // ------------------------------
    struct Cat{
        name : &'static str
    }
    // -------------------------------
    trait Animal{
                                 // big Self ( capitial S )
        fn maker(name: &'static str) -> Self;

        fn name(&self)->&'static str;

        // default implementation inside trait
        fn talk(&self){
            println!("{} can\'t talk",self.name());
        }
    }

    impl Animal for Human{

        fn maker(name: &'static str) -> Human{
            Human { name: name }
        }
        fn name(&self) -> &'static str {
            self.name
        }

        // human talk function override Animal talk function 
        // Own implementation
        fn talk(&self) {
            println!("{} can talk..!",self.name);
        }
    }

    impl Animal for Cat{
        fn maker(name: &'static str) -> Cat {
            Cat { name: name }
        }
        fn name(&self) -> &'static str {
            self.name
        }
        
        // Cat talk function override Animal talk function 
        // Own implementation
        fn talk(&self) {
            println!("{} sound meow..!",self.name);
        }
    }
    // let persion = Human{name: "Student..!"};
    // let persion = Human::maker("Alive");
    let persion:Human = Animal::maker("Alive");
    persion.talk();

    let cat  = Cat{name:"vatiyan"};
    cat.talk();
    // println!("Human traits : {:?}",);
    // -------------------------------------------------------------------------------

    // mode-2
    // sum all value in vector
    // T is generic type ( #What #type of args pass )
  
    trait Adding<T> {
        fn sum(&self)->T;
    }

    impl Adding<u8> for Vec<u8>{
        fn sum(&self) ->u8 {
            let mut tot:u8 = 0;
            for tm in self
                {
                    tot += *tm;
                }
            return  tot;
        }
    } 
    let list = vec![1,2,3,4,5];
    println!("{}",list.sum());
    
}

// instentent function
// static function

// ::fun() ---> factory function