use regex::Regex;
use lazy_static::lazy_static;

lazy_static!{
    static ref PUZZLE_CAPTURE: Regex = Regex::new(r"Valve (?P<label>-?.{2}) has flow rate=(?P<rate>-?\d+); [tunnels]+ [leads]+ to [valves]+(?P<others>-?.+)").unwrap();
}

#[test]
fn day_16_regex() {
    let example = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB";
    let captures = PUZZLE_CAPTURE.captures(example).unwrap();

    assert_eq!("AA", &captures["label"]);
    assert_eq!("0", &captures["rate"]);
    assert_eq!(" DD, II, BB", &captures["others"]);

    let example = "Valve HH has flow rate=22; tunnel leads to valve GG";
    let captures = PUZZLE_CAPTURE.captures(example).unwrap();

    assert_eq!("HH", &captures["label"]);
    assert_eq!("22", &captures["rate"]);
    assert_eq!(" GG", &captures["others"]);
}
