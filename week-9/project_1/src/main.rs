use std::io::Write;



fn main() {
    println!("Hello, This is a program that saves the categories of drinks for the Nigerian Breweries PLC");
    let mhmm = "Nigerian Breweries\n";
    let header= "LAGER,STOUT,NON-ALCOHOLIC\n";
    let first= ",,\n";
    let line_1="33 Export,Legend,Maltina\n";
    let line_2="Desperados,Turbo King,Amstel Malta\n";
    let line_3="Goldberg,Williams,Malta Gold\n";
    let line_4="Gulder,,Fayrouz\n";
    let line_5="Heineken,,\n";
    let line_6="Star,,\n";

    let mut brew=std::fs::File::create("Nigerian_Brewery_categorylist.csv").expect("Failed to create file");   
    brew.write_all(mhmm.as_bytes()).expect("Failed to write");
    brew.write_all(header.as_bytes()).expect("Failed to write");
    brew.write_all(first.as_bytes()).expect("Failed to write");
    brew.write_all(line_1.as_bytes()).expect("Failed to write");
    brew.write_all(line_2.as_bytes()).expect("Failed to write");
    brew.write_all(line_3.as_bytes()).expect("Failed to write");
    brew.write_all(line_4.as_bytes()).expect("Failed to write");
    brew.write_all(line_5.as_bytes()).expect("Failed to write");
    brew.write_all(line_6.as_bytes()).expect("Failed to write");

    println!("File created and content added");
}
