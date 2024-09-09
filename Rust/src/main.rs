// fn main() {
   // let arr = [1 , 2, 3, 4];
   // let slice = &arr[1..4];
   // println!("{:?}", slice);
   // println!("{}", slice.len());

   // let str = " Hello Cargo"; //creates a variable
   // println!("{}", str);

   // let string = String::from(str); // creates an object
   // println!("{}", string.);

   // let slice = &string[6..];
   // println!("{} {}", slice, string.len());
   // let name = String::from("Ali");
   // //let number = 6;
   // let student = Student{name, rollnumber : 4};
   // student.print_name();
   // struct Student
   // {
   //    name : String,
   //    rollnumber : u64
   // }

   // impl Student {
   //    fn print_name(&self)
   //    {
   //       println!("{} {}", self.name, self.rollnumber);
   //    }
       
   // }



//}



fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

pub trait Summary {
   fn summarize(&self) -> String;
}


pub struct Tweet {
   pub username: String,
   pub content: String,
   pub reply: bool,
   pub retweet: bool,
}

impl Summary for Tweet {
   fn summarize(&self) -> String {
       format!("{}: {}", self.username, self.content)
   }
}

