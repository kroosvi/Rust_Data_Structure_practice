
mod  algorithms{
    //---------------------------Heap---------------------//
    //Note: Tree base heap are more memory used than the array based
    //This I would try to use build both in rust
    // Website to review: https://www.programiz.com/dsa/heap-data-structure
    pub fn min_heap(original_vec:&mut [u64]){
        let mut parent_node :uszie = original_vec.len().div_ceil(2) - 1

        //heapify
        heapify(original_vec,parent_node);
    }

    //The LEFT  child is going to be from the 2 x (selected node) + 1
    //The RIGHT child is going to be from the 2 x (selected node) + 2

    //min heap keep it in mind
    //There is times there are no childs or there are no right child
    fn heapify(original_vec:&mut [u64],starting_node : usize){
        let parent_node : usize = starting_node;
        let left_child:usize = 2 * parent_node + 1;
        let right_child:usize= left_child + 1;
        let mut is_there_right_child : bool;

        //start with the smallest node being the parent
        let mut smallest_node : usize = parent_node;

        //Test for the case if there is no left child
        if(left_child > original_vec.len()){

        }else if(left_child + 1 <= original_vec.len()){//Needed to be tested
            is_there_right_child = true;
        }//if there is a right child


    }

    fn compare_and_swap(mut current_node : usize,node_that_needed_to_be_test : usize){

    }
    //----------------------Heap--------------------------//
}

fn main() {
    let mut array : Vec<u64> = vec![2,3,5,6,8,6,7,8];//As right now there is a right child

    let algorithm = algorithms::min_heap(&mut array);

}


