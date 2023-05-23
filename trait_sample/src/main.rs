use std::any::Any;
trait Animal:Any{
    fn sound(&self, logo:&str);
    fn as_any(&self)->&dyn Any;
} 

#[derive(Debug)]
struct Cat{
    name:String,
    age:u8,
}

impl Animal for Cat{
    fn sound(&self, logo:&str){
        let s = self.name.clone() + ":"+logo;
        println!("Cat make sound:{}", s)
    }
    fn as_any(&self)->&dyn Any{
        self
    }
}

#[derive(Debug)]
struct Dog{
    name:String,
    age:u8,
}

impl Dog{
    fn new(n:String, a:u8)->Self{
        Dog{
            name:n,
            age:8
        }
    }

    fn name(&self)->&str{
       &self.name 
    }

    fn set_age(&mut self, a:u8){
       self.age = a;
    }
}

impl Animal for Dog{
    fn sound(&self, logo:&str){
        let s = self.name.clone() + ":"+logo;
        println!("Dog make sound:{}", s)
    }

    fn as_any(&self)->&dyn Any{
        self
    }
}

fn make_sound(a: &dyn Animal){
    a.sound("win");
}

fn main() {
    // basic read and update
    let c:Cat = Cat{name:String::from("kitty"), age:2};
    println!("{:?}", c);
    let mut d = Dog::new("doggy".to_string(), 3);
    println!("{:?}", d);
    println!("dog name:{}", d.name());
    d.set_age(10);
    println!("Update age, {:?}", d);

    //cast animal to trait 
    make_sound(&c); 

    let da : &dyn Animal = &Dog::new("puppy".to_string(), 4);
    make_sound(da);

    //use box to transfer ownership
   let animalVect = vec![
        Box::new(c) as Box<dyn Animal>,
        Box::new(d) as Box<dyn Animal>,
   ]; 

   // downcast from trait to object, it should be transfer to Any first
   for item in animalVect{
        item.sound("bark");
        if let Some(cat) = item.as_any().downcast_ref::<Cat>(){
            println!("Get cat from vect:{:?}", cat);
        }else if let Some(dog) = item.as_any().downcast_ref::<Dog>(){
            println!("Get dog from vect:{:?}", dog);
        }
   }

}
