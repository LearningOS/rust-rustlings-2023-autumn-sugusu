// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.


#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice =&a[1..4];//这里的nice_slice是一个切片，切片是一个指向数组的指针，指向数组的某一部分，这里的nice_slice指向a数组的第二个元素到第四个元素


    assert_eq!([2, 3, 4], nice_slice)//assert_eq!函数是断言函数，如果两个参数不相等，就会报错
}
