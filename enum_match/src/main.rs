#[warn(dead_code)]
//use Debug to print in {:?}
#[derive(Debug)]
enum Book{
    Papery(u32),
    Electronic{url:String, price:i32}
}


fn enum_match(){
    let b:Book = Book::Electronic{url:String::from("a.b.c"), price:188};
    println!("{:?}", b);

    match b{
        Book::Electronic{url, price}=>{
            println!{"{}, {}", url, price}
        },
        Book::Papery(i)=>{
            println!{"{}", i}
        },
        _ => println!("nothing")
    }

    let c = Book::Papery(38);
    match c{
        Book::Electronic{url, price}=>{
            println!{"{}, {}", url, price}
        },
        Book::Papery(i)=>{
            println!{"{}", i}
        },
        _ => println!("nothing")
    }
    
}


fn option_match(){
    let mut v = Some("v1");

    match v{
        Some(v)=>{
            println!("value in option:{}", v)  
        },
        None=>{
            println!("None")
        }
    }

    if let Some(value) = v{
       println!{"if let suger:{}", value} 
    }

    v = Some("v2");
    if let Some("v1") = v{
       println!{"if let suger2:{}", "v1"} 
    }else{
        println!("if let suger2, not v1")
    }

    v = None;
    match v{
        Some(v)=>{
            println!("value in option:{}", v)  
        },
        None=>{
            println!("None")
        }
    }

    if let Some(value) = v{
       println!{"if let suger:{}", value} 
    }else{
        println!("if let suger, None")
    }
}


fn result_match(){
    let r1 = foo(1);
    match r1{
        Ok(v)=>{
            println!("call OK, v:{}", v)
        },
        Err(e)=>{
            println!("call Err, e:{}", e)
        }
    }

    let r2 = foo(3);
    match r2{
        Ok(v)=>{
            println!("call OK, v:{}", v)
        },
        Err(e)=>{
            println!("call Err, e:{}", e)
        }
    }
}


fn main() {
    //enum_match();
    //option_match();
    //result_match();
}
