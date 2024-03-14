use ic_cdk::query;
use std::cell::RefCell;

thread_local! {
    static COUNTER : RefCell<u32> = RefCell::new(0);
}

#[query]
fn whoami() -> String {
    let caller = ic_cdk::caller();
    caller.to_string()
}

//#[macro_use]
extern crate serde;
//use std::str::FromStr;

//use ic_cdk_macros::*;
//use ic_stable_structures::memory_manager::{VirtualMemory};
//use ic_stable_structures::{Cell, DefaultMemoryImpl};
//use candid::types::Type;
use candid::CandidType;
// use std::{collections::HashMap, string::String};
//use email_address::EmailAddress;


//type Memory = VirtualMemory<DefaultMemoryImpl>;
//type IdCell = Cell<u64, Memory>;


// Skill struct
#[derive(Clone, serde::Deserialize, serde::Serialize, CandidType)]

struct Skill {
    name: String,
    description: String,
    proficiency: f64,
    verified: bool,
}

// Define User struct
#[derive(Clone, serde::Deserialize, serde::Serialize, CandidType)]
struct User {
    name: String,
    skills: Vec<Skill>,
}