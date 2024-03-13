use candid::Deserialize
use ic_cdk::{query, update};
use std::cell::RefCell;

thread_local! {
    static COUNTER : RefCell<u32> = RefCell::new(0);
}

#[query]
fn whoami() -> String {
    let caller = ic_cdk::caller();
    caller.to_string()
}

#[query]
fn get_counter() -> u32 {
    COUNTER.with(|counter| *counter.borrow())
}

#[update]
fn inc_counter() -> u32 {
    COUNTER.with(|counter| {
        *counter.borrow_mut() += 1;
        *counter.borrow()
    })
}

#[macro_use]
extern crate serde;
extern crate ic_stable;


/*use candid::{Decode, Encode};
use ic_stable_structures::memory_manager::{MemoryId, Memory, Memory_Manager};
use ic_stable_structures::{BoundedStorable, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};
//use ic_stable_structures::{Memory};*/
use ic_stable::{StableDeref, Abomonation};
use std::borrow::Cow;
use candid::{Decode, Encode};
use ic_stable_structures::memory_manager::{MemoryId, Memory_Manager}; 
//use std::{cell::RefCell};
use ic_stable_structures::StableBTreeMap;
//use ic_stable_structures::BoundedStorable; 
use ic_stable_structures::Storable;
use std::string::String; 

// Define Skill struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct Skill {
    name: String,
    description: String,
    proficiency: f64,
    verified: bool,
}

impl Storable for Skill {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

trait BoundedStorable {
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(bytes: &[u8]) -> Option<Self> where Self: Sized;
}

impl BoundedStorable for Skill {
    fn to_bytes(&self) -> Vec<u8> {
        // Implement to_bytes for Skill
        self.to_candid_bytes().unwrap()
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        // Implement from_bytes for Skill
        Decode!(&bytes, Skill).ok()
    }
}

// Ensure String satisfies the ic_stable_ trait bound
impl ic_stable::StableDeref for String {}
impl<'_a> ic_stable::Abomonation for &'_a String {}

impl BoundedStorable for String {
    fn to_bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        String::from_utf8(bytes.to_vec()).ok()
    }
}

// Define Job struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct Job {
    title: String,
    description: String,
    requirements: Vec<String>,
    employer: String,
}

// Define User struct
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct User {
    name: String,
    skills: Vec<Skill>,
    // Add other user-related fields as needed
}

// Implement functionality for managing skills
impl User {
    fn add_skill(&mut self, skill: Skill) {
        self.skills.push(skill);
    }

    fn update_skill(&mut self, skill_name: &str, new_description: String, new_proficiency: f64) -> Result<(), String> {
        for skill in &mut self.skills {
            if skill.name == skill_name {
                skill.description = new_description;
                skill.proficiency = new_proficiency;
                return Ok(());
            }
        }
        Err("Skill not found".to_string())
    }

    fn delete_skill(&mut self, skill_name: &str) -> Result<(), String> {
        let index = self.skills.iter().position(|skill| skill.name == skill_name);
        if let Some(index) = index {
            self.skills.remove(index);
            Ok(())
        } else {
            Err("Skill not found".to_string())
        }
    }

    fn verify_skill(&mut self, skill_name: &str) -> Result<(), String> {
        for skill in &mut self.skills {
            if skill.name == skill_name {
                skill.verified = true;
                return Ok(());
            }
        }
        Err("Skill not found".to_string())
    }
}

// Define error types
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    Unauthorized { msg: String },
}

// Thread-local storage for managing users and jobs
thread_local! {
    static USERS: RefCell<StableBTreeMap<String, User, MemoryId>> = RefCell::new(
        StableBTreeMap::init(Memory_Manager.with(|m| m.borrow().get(MemoryId::new(0))))
    );
    /*static USERS: RefCell<StableBTreeMap<String, User, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))))
    );*/

    static JOBS: RefCell<StableBTreeMap<u64, Job, MemoryId>> = RefCell::new(
        StableBTreeMap::init(Memory_Manager.with(|m| m.borrow().get(MemoryId::new(1))))
    );

    /*static JOBS: RefCell<StableBTreeMap<u64, Job, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))))
    );*/
}

// Function to add a new user
#[ic_cdk::update]
fn add_user(name: String) {
    USERS.with(|users| {
        users.borrow_mut().insert(name.clone(), User { name, skills: Vec::new() });
    });
}

// Function to add a skill for a user
#[ic_cdk::update]
fn add_skill_for_user(user_name: String, skill: Skill) -> Result<(), Error> {
    USERS.with(|users| {
        let mut user_map = users.borrow_mut();
        let user = user_map.get_mut(&user_name).ok_or(Error::NotFound { msg: "User not found".to_string() })?;
        user.add_skill(skill);
        Ok(())
    })
}

// Function to update a skill for a user
#[ic_cdk::update]
fn update_skill_for_user(user_name: String, skill_name: String, new_description: String, new_proficiency: f64) -> Result<(), Error> {
    USERS.with(|users| {
        let mut user_map = users.borrow_mut();
        let user = user_map.get_mut(&user_name).ok_or(Error::NotFound { msg: "User not found".to_string() })?;
        user.update_skill(&skill_name, new_description, new_proficiency).map_err(|err| Error::NotFound { msg: err })
    })
}

// Function to delete a skill for a user
#[ic_cdk::update]
fn delete_skill_for_user(user_name: String, skill_name: String) -> Result<(), Error> {
    USERS.with(|users| {
        let mut user_map = users.borrow_mut();
        let user = user_map.get_mut(&user_name).ok_or(Error::NotFound { msg: "User not found".to_string() })?;
        user.delete_skill(&skill_name).map_err(|err| Error::NotFound { msg: err })
    })
}

// Function to verify a skill for a user
#[ic_cdk::update]
fn verify_skill_for_user(user_name: String, skill_name: String) -> Result<(), Error> {
    USERS.with(|users| {
        let mut user_map = users.borrow_mut();
        let user = user_map.get_mut(&user_name).ok_or(Error::NotFound { msg: "User not found".to_string() })?;
        user.verify_skill(&skill_name).map_err(|err| Error::NotFound { msg: err })
    })
}

// Function for employers to add a new job
#[ic_cdk::update]
fn add_job(job: Job) {
    JOBS.with(|jobs| {
        let mut job_map = jobs.borrow_mut();
        let id = job_map.len() as u64;
        job_map.insert(id, job);
    });
}

// Function for users to get notified of available jobs
#[ic_cdk::query]
fn get_available_jobs(user_name: String) -> Result<Vec<Job>, Error> {
    let mut available_jobs = Vec::new();
    JOBS.with(|jobs| {
        let job_map = jobs.borrow();
        for (_, job) in job_map.iter() {
            // Check if user's skills match job requirements
            let user_has_skills = USERS.with(|users| {
                let user_map = users.borrow();
                if let Some(user) = user_map.get(&user_name) {
                    for requirement in &job.requirements {
                        if !user.skills.iter().any(|skill| skill.name == *requirement) {
                            return false;
                        }
                    }
                    true
                } else {
                    false
                }
            });
            if user_has_skills {
                available_jobs.push(job.clone());
            }
        }
    });
    if available_jobs.is_empty() {
        Err(Error::NotFound { msg: "No jobs available for the user's skills".to_string() })
    } else {
        Ok(available_jobs)
    }
}

// Function for employers to rate a user after completing a job
#[ic_cdk::update]
fn rate_user(user_name: String, rating: f64) -> Result<(), Error> {
    // Implement rating functionality
    // For demonstration purposes, returning Ok(()).
    Ok(())
}

//Provides candid interface.
ic_cdk::export_candid!();
