use quick_js::Context;

fn main() {
    println!("Starting...");
    let context = Context::new().unwrap();
    println!("Result: {:?}", context.eval("1+34").unwrap());
}
