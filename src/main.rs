use std::str::FromStr;

type ReadFunction<T> = fn() -> Option<T>;

fn main() {
    let number: u32 = validate_read(read_number);
    let a: String = get_factoradic_num(number);
    println!("{a}");
}

fn read_string() -> Option<String> {
    let mut line: String = String::new();
    println!("Enter positive base 10 number less than 36288000: ");
    match std::io::stdin().read_line(&mut line) {
        Ok(_) => Some(line.trim().to_string()),
        Err(_) => None,
    }
}

fn read_number() -> Option<u32> {
    let input: String = read_string()?;
    let out = u32::from_str(&input).ok()?;
    if out >= 36288000 {
        return None;
    }
    Some(out)
}

fn validate_read<T>(func: ReadFunction<T>) -> T{
    loop {
        if let Some(a) = func(){
            break a;
        }
    }
}

struct State {
    out_val: u8,
    state_val: u32,
}

fn get_factoradic_num(in_num: u32) -> String {
    get_fac_array(in_num)
        .iter()
        .rev()
        .scan(State{ out_val: 0, state_val: *&in_num }, |state, x| {
            state.out_val = (state.state_val / x) as u8;
            state.state_val %= x;
            Some(state.out_val.to_string())
        })
        .collect()
}

fn get_fac_array(num: u32) -> Vec<u32> {
    (1..).scan(1, |state, i| {
        *state *= i;
        if *state > num{
            return None;
        }
        Some(*state)
    }).collect()
}