mod days;

fn main() {
    // execute_all()
    days::day_15::execute()
}

fn execute_all() {
    days::day_1::execute();
    days::day_2::execute();
    days::day_3::execute();
    days::day_4::execute();
    days::day_5::execute();
    days::day_6::execute();
    // days::day_7::execute();
    days::day_8::execute();
    days::day_9::execute(2);
    days::day_9::execute(10);
    days::day_10::execute();
    days::day_11::execute(20, false);
    days::day_11::execute(10000, true);
    days::day_12::execute();
    days::day_13::execute();
    days::day_14::execute(false);
    days::day_14::execute(true);
    days::day_15::execute()
}
