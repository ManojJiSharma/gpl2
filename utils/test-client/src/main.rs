fn main() {
    let sdk_dep = gemachain_sdk::signature::Signature::default();
    println!("Yes have some sdk_dep {:?}", sdk_dep);
    let memo_dep = gpl_memo::id();
    println!("Yes have some memo_dep {:?}", memo_dep);
    let token_dep = gpl_token::id();
    println!("Yes have some token_dep {:?}", token_dep);
    let token_swap_dep = gpl_token_swap::id();
    println!("Yes have some token_swap_dep {:?}", token_swap_dep);
}
