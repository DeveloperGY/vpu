pub struct MEM<T> {
    pub arr: Box<[T]>
}

impl<T> MEM<T> {
    pub fn new(capacity: u8) -> MEM<T> {
        let mem_vec: Vec<T> = Vec::with_capacity(capacity as usize);
        let mem_arr: Box<[T]> = mem_vec.into_boxed_slice();
        MEM {arr: mem_arr}        
    }
}