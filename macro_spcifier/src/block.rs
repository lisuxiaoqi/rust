macro_rules! block_wrapper {
    ($block:block) => {
        println!("Before Block");
        $block;
        println!("End Block");
    };
}

macro_rules! block_exec {
    ($block:block) => {
        println!("Before Block");
        (|| $block)();
        println!("End Block");
    };
}

pub fn block_wrapper_test() {
    block_wrapper!({ println!("Inner block logic") });
}

pub fn block_exec_test() {
    block_exec!({
        println!("Inner block logic");
    });
}
