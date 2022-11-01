pub fn run(){

  println!(" Hello ,world from print");
  println!("{}", 1);
  println!("{} is {}st in class", "Dawood",1);
  //positional arguments
  println!("{0} is from {1} and {0} likes to {2}","Dawood", "Kashmir","code");
// names arguments
  println!("{name} is from {place} and {name} likes to {activity}", name ="Dawood", place="Kashmir", activity="code");
  //placeholder traits
  println!("Binary : {:b} Hex:{:x} Octal: {:o}",10,10,10);
  println!("{:?}", (12,true,"hello"));

}