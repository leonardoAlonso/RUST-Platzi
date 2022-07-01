fn main() {

   let mut nombres: Vec<String> = Vec::new();


   for i in 0..3{
       println!("Add your name: ");
       let mut nombre = String::new();
       std::io::stdin().read_line(&mut nombre).unwrap();
    
       nombres.push(nombre);
   }


   println!("{:?}", nombres);
   println!("{}", nombres[0]);


   for nombre in nombres{
    println!("El nombre es: {}", nombre);
   }

}
