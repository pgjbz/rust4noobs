fn main() {
    let array: [u8; 10] = [1,2,3,4,5,6,7,8,9,10];
    
    for i in 0..array.len(){
        if array[i] % 2 == 0 {
            println!("{} é multiplo de dois",array[i]);    
        }
        else{
            println!("{} não é multiplo de dois",array[i]);    
        }
        
    }
}