
// use std::collections::HashMap;

//
//commit
//commitonpractise
// use std::collections::Vec;
use std::fmt::Debug;
fn main() {

//  let temp = Temperature {
//         degrees:40.3
//     };
//    let va = temp.show_temp() ;

//    let temp_2 = Temperature::freezing();
//    //temp_2 is an instance of the struct Temperature
//    //so you can now call its methods dot operator 
//    temp_2.show_temp();
   
//    let mut vec = Vec::new();


//    vec.push(1.2);
  
//    vec.push(2.2);

//    vec.push(1);
//    vec.push(2);


//    let mut vec1 = vec![1, 2, 3];

//    vec1.push(4);
//    vec1.pop();


//    let mut vec = Vec::with_capacity(5);

//        vec.resize(5, 0);


//        let vec = vec![0; 5];


//         for i in vec {
//             println!("{}", i);
//         }

//        let collected_iterator: Vec<i32> = (0..10).collect();


//        let mut data = HashMap::new();


//        data.insert("name",20);
//        data.insert("age",20);


    //    let value = data.get("name");

    //    for (key,value) in data.iter() {
    //        println!("{:?},{:?}", key, value);
    //    }

    //    for key in data.keys() {
    //     println!("{:?}", key);
    //    }

    //    for value in data.values(){
    //     println!("{:?}", value);
    //    }



//     let apple = Brands::Apple(4000);
// let samsung = Brands::Samsung(3000);


    // match apple {
    //   Brands::Apple(amount)=> println!("{}",amount),
    //   _=> (),
    // }


    // match checked_division(8,0) {
    //     Ok(result) => println!("{}", result),
    //     Err(e) => println!("{}", e)
    // }

    // let res = check(8,0) ;
    let res = MyEnum::Var1;
    print_my_enum(res);
    print_my_enum(res);

    foo::<i32,i32>(8,6);
    foo::<f32,f64>(8.1,4.5);
    foo::<&str,String>("name","MYName".to_owned());

    let mystruct_string: MyStruct<String> = MyStruct { content: "A String".to_string() };
    println!("{}", mystruct_string.content);

    let mystruct_i32: MyStruct<&str> = MyStruct { content: "my name" };
    println!("{}", mystruct_i32.content);

    let my_struct_tuple = GenericVal::<i32>(8);
    println!("{}", my_struct_tuple.0);

    let gen_val_struct = GenVal::<i32>{ gen_val: 92};
    println!("{}", gen_val_struct.gen_val);
    println!("{}", gen_val_struct.value());

    // calling the associated function over GenVal
    println!("{}", GenVal::<i32>::gen_fn::<i32>(32));
    
    //similar way i can call it for multiple types
    println!("{}", GenVal::<i32>::gen_fn::<&str>("name"));
    println!("{}", GenVal::<i32>::gen_fn::<f64>(32.32));


    let new_instance = GenVal::<i32>::new(4);
    println!("{}", new_instance.gen_val);

    let new_instance = GenVal::<&str>::new("name");
    println!("{}", new_instance.gen_val);

    let new_instance = GenVal::<f64>::new(7.5);
    println!("{}", new_instance.gen_val);

    // let example = Example::<i32,String,bool>::Var3(3,"this is a string".to_string(),true);
    // choose either one example   

      // now i can call Example on any type that i wanted 
    let example = Example::<i32,i32,i32>::new(2,3,4,"var1") ;
            //or
    let example = Example::<i32,bool,String>::new(2,true,"name".to_string(),"var3") ;
            //or    
    let example = Example::<i32,f64,bool>::new(2,4.5,false,"var2") ;
    
  
    match example{
        Example::Var3(data1,data2,data3)=> println!("{:?},{:?},{:?}",data1,data2,data3),
        Example::Var2(data1,data2)=> println!("{:?},{:?}",data1,data2),
        Example::Var1(data1)=> println!("{:?}",data1),
        _=> (),
    }
  
 
    let  clo = move| x:i32|{

      let  y=x +1 ;
       println!("{}", y);
    };
    let   value = 5;
    // clo(value);
    let newval = value;
   let an = get_value(value);
   let ans = val_get(value);
    println!("{}",an);
    println!("{}",ans);
    println!("{}",value)

   
}


//closure borrows by deafult
fn val_get(value:i32)->i32 {
    value+5
}

fn get_value(value:i32) -> i32{
    value+10
}
  
#[derive(Debug,Clone,Copy)]
enum MyEnum{
    Var1,
    Var2,
    Var3,
}


struct GenVal<T> {
    gen_val: T,
}


enum Example<A,B,C>{
    Var1(A),
    Var2(A,B),
    Var3(A,B,C),
    Invalid

}



impl <A,B,C>Example<A,B,C>{

     fn new(a:A,b:B,c:C,input:&str)->Self{

     let res =   match input {
            "var1"=> Example::<A,B,C>::Var1(a),
            "var2"=> Example::<A,B,C>::Var2(a,b),
            "var3"=> Example::<A,B,C>::Var3(a,b,c),
            _=> Example::<A,B,C>::Invalid
        };
       
        res
     }

}

impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }

fn new(a:T)-> Self{
      Self{
     gen_val:a
      }
 }

 fn gen_fn<U>(a:U)->U{
     a 
 }

}



struct GenericVal<T>(T); 



fn print_my_enum(my_enum:MyEnum) {
     
let mut  i = 3;

loop{

println!("{:?}", my_enum);
i=i-1;
if i==0{
    break;
}

}
   
}


// fn foo<T>(arg: T) { ... }



fn foo<T:Debug,U:Debug>(arg1: T,arg2:U) { 
    println!("{:?}", arg1);
    println!("{:?}", arg2);
 }

    struct MyStruct<A> { 
        content: A 
    }
 




//...



// enum Option<T> {
//     Some(T),
//     None
// }


// enum Result<T,E> {
//     Ok(T),
//     Err(E)
// }

// enum Brands {
//  Apple(i32),
//  Samsung(i32),
// }

// let mut  some_variable:Option<i32> ; 
// some_variable = Some(3);
// some_variable = None 

// let mut  some_variable:Option<f64>; 
// some_variable = Some(3.3);
// some_variable = None 

// let mut  some_variable:Option<String>; 
// some_variable = Some(String::from("name"));
// some_variable = None 


// fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
//     if divisor == 0 {
//        None
//     } else {
//       Some(dividend / divisor)
//     }
// }


fn checked_division(dividend: i32, divisor: i32) -> Result<i32,String> {
    if divisor == 0 {
       Err(String::from("can't divide by zero "))
    } else {
      Ok(dividend / divisor)
    }
}

fn check(a:i32,b:i32) -> Result<(), String>{
    let result = checked_division(a,b)?;
     println!("{}",result);
     println!("i didn't got executed");
    Ok(())
}
  
// match checked_division(8,2) {
//     Some(result) => println!("{}", result),
//     None => println!("divisor can't be zero")
// }



// enum Direction{
//     Up,
//     Down,
//     Right,
//     Left,
// }


// impl Direction {
//    fn new() -> Self{
//         Self{
//             Direction::Brown
//         }
//     }
//    fn print(&self) {
//         match self {
//             Direction::Up => println!("up"),
//             Direction::Down => println!("down"),
//             _ => 
//         }
//     }
// }


//    struct Temperature {
//    degrees:f64,
//    }
   
//    impl Temperature {
//     fn freezing() -> Self {
//      Self { degrees: 32.0}
//     }
//     fn show_temp(self)  {
//      println!("{:?}", self.degrees) ;
//    }
//    }
   





