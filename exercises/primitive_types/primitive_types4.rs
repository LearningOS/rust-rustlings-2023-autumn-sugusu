// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice =[a.get(1).unwrap(),a.get(2).unwrap(),a.get(3).unwrap()];// TODO: Finish this line so that the slice is a slice of the second & third elements of the array


    assert_eq!([2, 3, 4], nice_slice)//assert_eq!函数是断言函数，如果两个参数不相等，就会报错
}
