mod sh;

const MEANING_OF_LIFE:u8 = 42; // no fixed address

static mut Z:i32 = 12345;

fn main() {
    println!("meaning of life = {}", MEANING_OF_LIFE);
    
    unsafe {
        Z = 123;
        println!("ZZZZ = {}", Z);
    }

    sh::stack_and_heap();
}
