mod e_1;
mod e_10;
mod e_13;
mod e_14;
mod e_2;
mod e_3;
mod e_4;
mod e_5;
mod e_6;
mod e_7;
mod e_8;

fn main() {
    e_1::run();
    e_2::run();
    e_3::run();
    e_4::run();
    e_5::run(); // PERF:
    e_6::run();
    // seven::run(); // PERF:
    e_8::run();
    e_10::run();
    e_13::run();
    e_14::run();
}
