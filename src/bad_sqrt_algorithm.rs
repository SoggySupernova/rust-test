pub fn main(accuracy: i8) -> f32{
    let mut arr: Vec<f32> = (1..i32::pow(2, accuracy as u32)+1).map(|x| x as f32).collect();
    println!("Initializing array with {} elements",arr.len());
    println!("Array is {:?}",arr);
    let mut newarr: Vec<f32>;
    for _i in 0..accuracy-1 {
        newarr = arr.chunks_exact(2).map(|pair| {
            pair[0] / pair[1]
        }).collect();
        arr = newarr;
        println!("Array has {} elements",arr.len());
        println!("Array is {:.32?}",arr);
        std::thread::sleep(std::time::Duration::from_millis(10)); // prevent crashes
    }
    1.0 / arr[0]
}