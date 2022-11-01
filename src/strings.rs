pub fn run(){
  let mut hello = String::from("Hello ");
   hello.push('w');
   hello.push_str("orld!");
   println!("{}", hello);
   println!("{}",hello.len())
   hello.replace("world","there");
   assert_eq!(12,s.len());
}