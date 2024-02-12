fn main() {
    about_scopes()
}


fn about_scopes() {
    let x  = 1;
    println!("scope: 01 - x = {x}, X is available here because it's pertencent to this scope! ");
    {
        println!("scope: 02 - x = {x}, X is available here too because this scope that initiates with '{{' and ends with '}}' are a child scope and x was declared before the scope initalization represented by '{{'");

        let y = 2;

        println!("scope: 02 - y = {y}, Y are the same as X, existing in this scope.")
    }

    println!("scope: 01 - Now X are still available here, but not Y, because Y do not exist in this scope! x = {x}");
    println!("scope: 01 - Since Y are not available in this scope, after the '}}' in line 15, the y variable will be droped by rust, no longer being saved in the memory")
}
