use md5;

pub fn run() {
    let key = "ckczppom";

    p1(&key);
    p2(&key);
}

fn p1(key: &str) {
    let key = key.to_string();
    let mut secret_number = 1;

    loop {
        let secret_number_string = secret_number.to_string();

        let digest = md5::compute((key.clone() + &secret_number.to_string()).as_bytes());
        let digest = format!("{:x}", digest);

        if &digest[0..5] == "00000" {
            println!("Match! Secret number is: {secret_number_string}");
            break;
        } else {
            secret_number += 1;
        }
    }
}

fn p2(key: &str) {
    let key = key.to_string();
    let mut secret_number = 1;

    loop {
        let secret_number_string = secret_number.to_string();
        let current_key = key.clone() + &secret_number_string[..];

        let digest = md5::compute(current_key);
        let digest = format!("{:x}", digest);

        if &digest[0..6] == "000000" {
            println!("Match! Secret number is: {secret_number_string}");
            break;
        } else {
            secret_number += 1;
        }
    }
}
