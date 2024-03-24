struct Algorithms;
impl Algorithms{
    pub fn new() -> Self{
        Self
    }

    //---------------------------Heap---------------------//
    //Note: Tree base heap are more memory used than the array based
    //This I would try to use build both in rust

    pub fn min_heap(original_vec:&mut [u64]){
        //heapify
        Self::heapify(original_vec);
    }

    fn swap(original_vec:&mut [u64], firstNode : u32){

    }

    fn heapify(original_vec:&mut [u64]){
        let mut starting_node = original_vec.len()/2;
        starting_node = starting_node.div_ceil(0) - 1;
    }
    //----------------------Heap--------------------------//
}

fn main() {
    let mut array : Vec<u64> = vec![2,3,5,6,8,9];

    let algorithm = Algorithms::new();
}


