fn main() {
   let arr = [1 , 2, 3, 4];
   let slice = &arr[1..4];
   println!("{:?}", slice);
   println!("{}", slice.len());

   let str = " Hello Cargo"; //creates a variable
   println!("{}", str);

   let string = String::from(str); // creates an object
   println!("{}", string.);

   let slice = &string[6..];
   println!("{} {}", slice, string.len());


}
