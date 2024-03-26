
fn make_func(){

    let i = 1.0;

    fn count() {

        i = i + 1.0;
        print i;
    }
    
    return count;
}


let test = make_func();
test();
test();
