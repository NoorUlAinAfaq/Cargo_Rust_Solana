fn main() {
   let arr = [1 , 2, 3, 4];
   let slice = &arr[1..4];
   println!("{:?}", slice);
   println!("{}", slice.len());

   let str = " Hello Cargo";
   println!("{}", str);

   let string = String::from(str);
   println!("{}", string);


}
