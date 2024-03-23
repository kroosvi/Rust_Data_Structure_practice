impl algorithms{
    //---------------------------Heap---------------------//
    //Note: Tree base heap are more memory used than the array based
    //This I would try to use build both in rust

    pub fn min_heap(original_vec:&mut [u64]){
        //heapify
        heapify();
    }

    fn swap(original_vec:&mut [u64], firstNode : u32){

    }

    fn heapify(original_vec:&mut [u64]){
        let mut starting_node = original_vec.len()/2;
        starting_node = starting_node.ceil() - 1;

    }
    //----------------------Heap--------------------------//
}

fn main() {
    let mut array = vec![2,3,5,6,8,9];

    let algorithm = algorithms::new();

    algorithm.min_heap(&array);
}


