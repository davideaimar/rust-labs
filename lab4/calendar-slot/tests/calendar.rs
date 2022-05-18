use std::str::FromStr;
use calendar_slot::calendar::Calendar;
use calendar_slot::timeslot::Timeslot;


#[test]
fn one_empty_calendar() {
    let bounds_a = Timeslot::from_str("08:00 18:00").unwrap();
    let bounds_b = Timeslot::from_str("09:00 19:00").unwrap(); // b is free all day

    let mut slots_a = vec![];
    slots_a.push(Timeslot::from_str("10:00 13:00").unwrap()); // a is busy from 10:00 to 13:00

    let cal_a = Calendar::new(slots_a, bounds_a);
    let cal_b = Calendar::new(vec![], bounds_b);

    let result = cal_a.free_slots_with(&cal_b, 30);
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].to_string(), "09:00 10:00");
    assert_eq!(result[1].to_string(), "13:00 18:00");
}

#[test]
fn both_empty_calendar() {
    let bounds_a = Timeslot::from_str("08:00 18:00").unwrap();
    let bounds_b = Timeslot::from_str("09:00 19:00").unwrap();

    let cal_a = Calendar::new(vec![], bounds_a);
    let cal_b = Calendar::new(vec![], bounds_b);

    let result = cal_a.free_slots_with(&cal_b, 30);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].to_string(), "09:00 18:00");
}

#[test]
fn one_full_calendar(){

    let bounds_a = Timeslot::from_str("08:00 18:00").unwrap();
    let bounds_b = Timeslot::from_str("09:00 19:00").unwrap();

    let mut slots_a = vec![];
    slots_a.push(Timeslot::from_str("8:00 13:00").unwrap()); // a is busy from 8 to 13
    slots_a.push(Timeslot::from_str("13:00 18:00").unwrap()); // a is busy from 13 to 18

    let mut slots_b = vec![];
    slots_b.push(Timeslot::from_str("9:00 10:15").unwrap()); // a is busy from 8 to 10

    let cal_a = Calendar::new(slots_a, bounds_a);
    let cal_b = Calendar::new(slots_b, bounds_b);

    let result = cal_a.free_slots_with(&cal_b, 30);
    assert_eq!(result.len(), 0);
}

#[test]
fn no_free_slots_long_enough(){

    let bounds_a = Timeslot::from_str("08:00 18:00").unwrap();
    let bounds_b = Timeslot::from_str("09:00 19:00").unwrap();

    let mut slots_a = vec![];
    slots_a.push(Timeslot::from_str("8:00 12:30").unwrap());
    slots_a.push(Timeslot::from_str("13:00 16:00").unwrap());
    slots_a.push(Timeslot::from_str("16:30 18:00").unwrap());

    let mut slots_b = vec![];
    slots_b.push(Timeslot::from_str("9:00 12:45").unwrap());
    slots_b.push(Timeslot::from_str("16:10 19:00").unwrap());

    let cal_a = Calendar::new(slots_a, bounds_a);
    let cal_b = Calendar::new(slots_b, bounds_b);

    let result = cal_a.free_slots_with(&cal_b, 30);
    assert_eq!(result.len(), 0);
}

#[test]
fn free_slots_begin_and_end_day(){

    let bounds_a = Timeslot::from_str("08:00 18:00").unwrap();
    let bounds_b = Timeslot::from_str("09:00 19:00").unwrap();

    let mut slots_a = vec![];
    slots_a.push(Timeslot::from_str("10:00 16:30").unwrap());

    let mut slots_b = vec![];
    slots_b.push(Timeslot::from_str("10:15 17:20").unwrap());

    let cal_a = Calendar::new(slots_a, bounds_a);
    let cal_b = Calendar::new(slots_b, bounds_b);

    let result = cal_a.free_slots_with(&cal_b, 30);
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].to_string(), "09:00 10:00");
    assert_eq!(result[1].to_string(), "17:20 18:00");
}

#[test]
fn exact_len_free_slot(){
    let bounds_a = Timeslot::from_str("08:00 18:00").unwrap();
    let bounds_b = Timeslot::from_str("09:00 19:00").unwrap();

    let mut slots_a = vec![];
    slots_a.push(Timeslot::from_str("9:20 17:30").unwrap());

    let mut slots_b = vec![];
    slots_b.push(Timeslot::from_str("10:15 17:20").unwrap());

    let cal_a = Calendar::new(slots_a, bounds_a);
    let cal_b = Calendar::new(slots_b, bounds_b);

    let result = cal_a.free_slots_with(&cal_b, 30);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].to_string(), "17:30 18:00");
    assert_eq!(result[0].get_duration(), 30);
}