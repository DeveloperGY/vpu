pub struct MEM<T> {
    pub arr: Box<[T]>
}

impl<T: Copy + Clone> MEM<T> {
    pub fn new(capacity: u8, default_value: T) -> MEM<T> {
        let mut mem_vec: Vec<T> = Vec::with_capacity(capacity as usize);
        for _x in 0..=capacity {
            mem_vec.push(default_value);
        }
        
        let mem_arr: Box<[T]> = mem_vec.into_boxed_slice();
        MEM {arr: mem_arr}        
    }
}