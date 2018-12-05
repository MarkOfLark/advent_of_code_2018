#[macro_use]
extern crate nom;

use nom::digit;
use nom::types::CompleteStr;
use std::str::FromStr;
use std::cmp::Ordering;
use std::collections::HashMap;


#[derive (Eq, PartialEq, Debug, PartialOrd)]
enum EventType {
    BeginShift(u32),
    FallAsleep,
    WakeUp,
}
impl Ord for EventType {
    fn cmp(&self, other: &EventType) -> Ordering {
        Ordering::Equal
    }
}

#[derive (Eq, PartialEq, Debug, PartialOrd, Ord)]
struct Event {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    event: EventType,
}


named!(begin_shift <CompleteStr,EventType>, do_parse!(
    tag!("Guard #") >>
    id: map_res!((digit), |CompleteStr(s)| u32::from_str(s)) >>
    (EventType::BeginShift(id))
));

named!(fall_asleep <CompleteStr,EventType>, do_parse!(
    tag!("falls asleep") >>
    (EventType::FallAsleep)
));

named!(wake_up <CompleteStr,EventType>, do_parse!(
    tag!("wakes up") >>
    (EventType::WakeUp)
));

named!(event_type <CompleteStr,EventType>, alt!(
    begin_shift | fall_asleep | wake_up
));


named!(parse_event <CompleteStr,Event>, do_parse!(
    tag!("[") >>
    year: map_res!(digit, |CompleteStr(s)| u32::from_str(s)) >>
    tag!("-") >>
    month: map_res!(digit, |CompleteStr(s)| u32::from_str(s)) >>
    tag!("-") >>
    day: map_res!(digit, |CompleteStr(s)| u32::from_str(s)) >>
    tag!(" ") >>
    hour: ws!(map_res!(digit, |CompleteStr(s)| u32::from_str(s))) >>
    tag!(":") >>
    minute: ws!(map_res!(digit, |CompleteStr(s)| u32::from_str(s))) >>
    ws!(tag!("]")) >>
    event: event_type >>
    (Event{year,month,day,hour,minute,event: event})
));


fn print_shift_map(event: &Vec<Event>) {
    println!("Date   ID     Minute");
    println!("              000000000011111111112222222222333333333344444444445555555555");
    println!("              012345678901234567890123456789012345678901234567890123456789");
    
    let mut last_printed_minute = 0;
    for ev in event.iter() {
        match ev.event {
            EventType::BeginShift(gid) => {
                if 0 != last_printed_minute {
                    for _ in last_printed_minute..60 {
                        print!(".");
                    }
                }
                print!("\n{:02}-{:02}  #{:<6}",ev.month,ev.day,gid);
                last_printed_minute = 0;
            },
            EventType::WakeUp => {
                for _ in last_printed_minute..ev.minute {
                    print!("#");
                }
                last_printed_minute = ev.minute;
            }
            EventType::FallAsleep => {
                for _ in last_printed_minute..ev.minute {
                    print!(".");
                }
                last_printed_minute = ev.minute;
            }
        }
    }
    for _ in last_printed_minute..60 {
        print!(".");
    }
    println!("")
}


fn print_shift_count(guard: &HashMap<u32,Vec<u32>>) {
    println!("ID     Minute");
    println!("        0 0 0 0 0 0 0 0 0 0 1 1 1 1 1 1 1 1 1 1 2 2 2 2 2 2 2 2 2 2 3 3 3 3 3 3 3 3 3 3 4 4 4 4 4 4 4 4 4 4 5 5 5 5 5 5 5 5 5 5");
    println!("        0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9");
    
    for g in guard.iter() {
        print!("\n#{:<6}",g.0);
        for c in g.1.iter() {
            print!("{:2}",c)
        }
    }
    println!("")
}


#[no_mangle]
pub fn aoc_solution(star: i32, input: &str) -> String {

    let mut event: Vec<Event> = Vec::new();
    for line in input.lines() {
        let e = parse_event(CompleteStr(line)).unwrap().1;
        event.push(e);
    }
    event.sort();

    //print_shift_map(&event);

    let mut record: HashMap<u32,Vec<u32>> = HashMap::new();

    let mut guard: Option<u32> = None;
    let mut last_updated_minute = 0u32;
    for ev in event.iter() {
        match ev.event {
            EventType::BeginShift(gid) => {
                guard = Some(gid);
                if !record.contains_key(&gid) {
                    record.insert(gid, vec![0;60]);
                }
                last_updated_minute = 0;
            },
            EventType::WakeUp => {
                if let Some(gid) = guard {
                    if let Some(v) = record.get_mut(&gid) {
                        for m in last_updated_minute..ev.minute {
                            v[m as usize] += 1;
                        }
                        last_updated_minute = ev.minute;
                    }
                }
            }
            EventType::FallAsleep => {
                if let Some(_) = guard {
                    last_updated_minute = ev.minute;
                }
            }
        }
    }
        
        //print_shift_count(&record);

    let mut total_mins = 0u32;
    let mut max_count = 0u32;
    let mut chosen_gid = 0u32;
    let mut sleepiest_minute = 0u32;
    for g in record.iter() {

        let guards_total_mins = g.1.iter().sum();

        // TODO do this in one step?
        let guards_max_count = *g.1.iter().max().unwrap();
        let guards_sleepiest_minute = g.1.iter().position(|&c| c == guards_max_count).unwrap() as u32;

        if 1 == star {
            if guards_total_mins > total_mins {
                chosen_gid = *g.0;
                total_mins = guards_total_mins;
                sleepiest_minute = guards_sleepiest_minute;
            }
        }
        else {
            if guards_max_count > max_count {
                chosen_gid = *g.0;
                max_count = guards_max_count;
                sleepiest_minute = guards_sleepiest_minute;
            }
        }
    }

    (chosen_gid*sleepiest_minute).to_string()
}
