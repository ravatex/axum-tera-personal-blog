use chrono::{DateTime, TimeDelta, Utc};
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

struct VisitorLogSmall {
    past_visitors: Vec<usize>,
    current: AtomicUsize,
    last_check: DateTime<Utc>,
    division: TimeDelta,
}

impl VisitorLogSmall {
    pub fn create_visitor_log(time_accuracy: TimeDelta) -> Self {
        VisitorLogSmall {
            past_visitors: Vec::new(),
            current: AtomicUsize::new(0),
            last_check: Utc::now(),
            division: time_accuracy,
        }
    }

    pub fn get_division(&self) -> TimeDelta {
        self.division
    }

    pub fn add_visitor(&mut self) -> usize {
        let now = Utc::now();
        if now > self.last_check + self.division {
            self.last_check += self.division;
            self.past_visitors.push(self.current.swap(0, SeqCst))
        }

        while now > self.last_check + self.division {
            self.last_check += self.division;
            self.past_visitors.push(0);
        }

        self.current.fetch_add(1, SeqCst) + 1
    }

    pub fn hour_visitor_log() -> Self {
        Self::create_visitor_log(TimeDelta::hours(1))
    }


    pub fn get_all_visitors(&self) -> usize {
        let count = self.past_visitors.iter().fold(0, |acc, x| acc + x);
        count + self.current.load(SeqCst)
    }
}

pub struct VisitorLog {
    visitors: Vec<DateTime<Utc>>,
}

impl VisitorLog {
    pub fn new() -> Self {
        VisitorLog {
            visitors: Vec::new(),
        }
    }

    pub fn add_visitor(&mut self) {
        self.visitors.push(Utc::now());
    }

    pub fn get_visitors_in_the_past(&self, time: TimeDelta) -> usize {
        self.get_visitors_since(Utc::now() - time)
    }

    pub fn get_visitors_since(&self, time: DateTime<Utc>) -> usize {
        let length_of_not_included = self.visitors.binary_search(&time).unwrap_or_default();
        self.visitors.len() - length_of_not_included
    }

    pub fn get_all_visitors(&self) -> usize {
        println!("{:?}", self.visitors);
        self.visitors.len()
    }
}
lazy_static! {
    static ref VISITORS: Arc<Mutex<VisitorLog>> = Arc::new(Mutex::new(VisitorLog::new()));
}

pub fn get_visitors() -> usize {
    match VISITORS.lock() {
        Ok(vis) => vis.get_all_visitors(),
        Err(_) => {
            println!("get visitors: thread panicked when holding lock");
            return 0;
        }
    }
}

pub fn increment_visitors() {
    match VISITORS.lock() {
        Ok(mut vis) => vis.add_visitor(),
        Err(_) => println!("get visitors: thread panicked when holding lock"),
    }
}
