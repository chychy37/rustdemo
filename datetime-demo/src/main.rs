use chrono::{DateTime, Duration, Local, NaiveDate, NaiveDateTime, Utc};

fn main() {
    // NaiveDate:			2020-12-12
    let a: NaiveDate = NaiveDate::from_ymd(2020, 12, 12);
    println!("NaiveDate:\t\t\t{:?}", a);

    // NaiveDateTime:		2020-12-12T12:12:12
    let b: NaiveDateTime = NaiveDate::from_ymd(2020, 12, 12).and_hms(12, 12, 12);
    println!("NaiveDateTime:\t\t{:?}", b);

    // DateTime<Local>:	    2021-01-06T16:03:54.532819+08:00
    let c: DateTime<Local> = Local::now();
    println!("DateTime<Local>:\t{:?}", c);

    // DateTime<Utc>:		2021-01-06T08:03:54.533404Z
    let d: DateTime<Utc> = Utc::now();
    println!("DateTime<Utc>:\t\t{:?}", d);

    let e: NaiveDateTime = Local::now().naive_local();
    let f: NaiveDateTime = Utc::now().naive_local();
    assert_ne!(e, f); // 2021-01-06T15:30:20.535461 != 2021-01-06T07:30:20.535464

    // checked_sub_signed or checked_add_signed
    let g: Option<NaiveDateTime> = e.checked_add_signed(Duration::hours(1));
    let h: Option<NaiveDateTime> = e.checked_add_signed(Duration::days(1));
    let i: Option<NaiveDateTime> = e.checked_add_signed(Duration::weeks(1));
    if g < h && h < i {
        println!("ok");
    }

    // timestamp:			1609949034
    let j = e.timestamp();
    println!("timestamp:\t\t\t{}", j);

    // NaiveDateTime:		2021-01-06T16:03:54
    let k: NaiveDateTime = NaiveDateTime::from_timestamp(j, 0);
    println!("NaiveDateTime:\t\t{:?}", k);

    // how to get now timestamp
    // timestamp:			1609920234
    let l = Local::now().timestamp();
    println!("timestamp:\t\t\t{}", l);
}
