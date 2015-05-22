extern crate rand;
use rand::Rng;
fn main() {
let mut wins=0;
let mut losses=0;
let mut draws =0;
let mut times =0;
let mut rock =0;
let mut paper=0;
let mut scissors=0;
let mut orock=0;
let mut opaper=0;
let mut oscissors=0;

while wins<1000000 && losses <1000000 {
let  mythrow = rand::thread_rng().gen_range(1, 4);
let otherthrow = rand::thread_rng().gen_range(1, 4);
match mythrow{
  1=>rock +=1,
  2=>paper +=1,
  3=>scissors+=1,
_=>{}
}
match otherthrow{
1=>orock +=1,
2=>opaper +=1,
3=>oscissors+=1,
_=>{}
}

match mythrow - otherthrow {
	0 => draws +=1,
       -1 => losses +=1,
	2 => losses +=1,
	1 => wins +=1,
       -2 => wins +=1,
    _ => {}
}
times+=1;
}

println!("Wins: {}", wins);
println!("Losses: {}", losses);
println!("Draws: {}", draws); 
println!("Total Throws {}",times);
println!("Rock: {} to {}",rock,orock);
println!("Paper: {} to {}",paper,opaper);
println!("Scissors: {} to {}",scissors,oscissors);
}
