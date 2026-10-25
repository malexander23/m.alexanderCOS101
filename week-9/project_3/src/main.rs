
use std::io::Read;
use std::io::Write;


fn main() {

    println!("Welcome\ns");
    let commisioner=vec!["Aigbogun Alamba Daudu","Murtala Afeez Bendu","Okorocha Calistus Ogbona","Adewale Jimoh Akanbi","Osazuwa Faith Etieye"];
    let mut com= std::fs::File::create("Commisioner file.csv").expect("Failed to create file");
    com.write_all("Commisioner\n".as_bytes()).expect("Failed to write line");
    for i in 0..commisioner.len(){
    com.write_all(format!("{}\n",commisioner[i]).as_bytes()).expect("Failed to write all");
    }
  let ministry=vec!["Internal Affairs","Justice","Defense","Power & Steel","Petroleum"];
  let mut min = std::fs::File::create("Ministry_file.csv").expect("Failed to create file");
  min.write_all("Minstry\n".as_bytes()).expect("Failed to write line");
  for i in 0..ministry.len(){
    min.write_all(format!("{}\n",ministry[i]).as_bytes()).expect("Failed to write all");
  }
 let zone=vec!["South West","North East","South South","South West","South East"];
 let mut z  = std::fs::File::create("Zones_file.csv").expect("Failed to create file");
  z.write_all("Zone\n".as_bytes()).expect("Failed to write line");
  for i in 0..zone.len(){
    z.write_all(format!("{}\n",zone[i]).as_bytes()).expect("Failed to write all");
  }

let mut min_1 = std::fs::File::open("Ministry_file.csv").expect("Failed to open file");
let mut content_1 :Vec<String>= Vec::new();
let mut cont = String::new();
min_1.read_to_string(&mut cont).expect("Failed to read to string");
for c in cont.lines(){
    let c_1=c.to_string();
content_1.push(c_1);
}




let mut com_1 = std::fs::File::open("Commisioner file.csv").expect("Failed to open file");
let mut comm_1 :Vec<String>= Vec::new();
let mut commi = String::new();
com_1.read_to_string(&mut commi).expect("Failed to read to string");
for a in commi.lines(){
let  b = a.to_string();
comm_1.push(b);
}
println!("{}",comm_1.len());


let mut zon = std::fs::File::open("Zones_file.csv").expect("Failed to open file");
let mut zon_1 :Vec<String>= Vec::new();
let mut zone_1 = String::new();
zon.read_to_string(&mut zone_1).expect("Failed to read to string");
for ze in zone_1.lines(){
    let z_1=ze.to_string();
zon_1.push(z_1);
}

let mut fin= std::fs::File::create("EFCC Merge.csv").expect("Failed to create file");
fin.write_all("EFCC convicted ministers\n ".as_bytes()).expect("Failed to write line");
fin.write_all("Commisioner,Ministry,Zone\n".as_bytes()).expect("Failed to write line");
for i in 1..comm_1.len(){
fin.write_all(format!("{},{},{}\n",comm_1[i],content_1[i],zon_1[i]).as_bytes()).expect("Failed to write all");
}
println!("Completed!");


}
