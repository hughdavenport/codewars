fn find_nb(m: u64) -> i32 {
    // your code
    let n: u64 = (((1f64 + (8f64 * (m as f64).sqrt())).sqrt() - 1f64) / 2f64) as u64;
    if 4u64 * m == n.pow(2) * (n+1).pow(2) {
        return n as i32;
    } else {
        return -1;
    }
}

fn find_nb_brute(m: u64) -> i32 {
    // your code
    let mut i: u64   = 1;
    let mut vol: u64 = 1;

    while vol < m {
        i += 1;
        vol += i.pow(3);

    }
    if vol == m {
        return i as i32;
    } else {
        return -1;
    }
}

fn testing(n: u64, exp: i32) -> () {
    assert_eq!(find_nb(n), exp);
    assert_eq!(find_nb_brute(n), exp);
}

//#[test]
fn basics_find_nb() {
    testing(4183059834009, 2022);
    testing(24723578342962, -1);
    testing(135440716410000, 4824);
    testing(40539911473216, 3568);
    testing(26825883955641, 3218);
}

fn main() {
    println!("{}", find_nb(1071225));
	basics_find_nb();
}
