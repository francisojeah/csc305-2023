// Tuples can be used as function arguments and as return values
fn reverse(pair: (i32, bool))->(bool, i32){
    // 'let' can be used to bind the members of a tuple to variables
    let (int_param, bool_param) = pair;
    (bool_param, int_param)
}

pub fn example(){

}