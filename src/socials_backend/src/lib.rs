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

extern crate serde;


use candid::CandidType;
use std::{collections::HashMap, string::String};



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

impl User {
    fn new(name: String) -> Self {
        User {
            name,
            skills: Vec::new(),
        }
    }

    fn add_skill(&mut self, skill: Skill) {
        self.skills.push(skill);
    }

    fn update_skill(&mut self, skill_name: &str, new_description: String, new_proficiency: f64) -> Result<(), String> {
        if let Some(skill) = self.skills.iter_mut().find(|s| s.name == skill_name) {
            skill.description = new_description;
            skill.proficiency = new_proficiency;
            Ok(())
        } else {
            Err("Skill not found".to_string())
        }
    }

    fn delete_skill(&mut self, skill_name: &str) -> Result<(), String> {
        if let Some(index) = self.skills.iter().position(|s| s.name == skill_name) {
            self.skills.remove(index);
            Ok(())
        } else {
            Err("Skill not found".to_string())
        }
    }

    fn verify_skill(&mut self, skill_name: &str) -> Result<(), String> {
        if let Some(skill) = self.skills.iter_mut().find(|s| s.name == skill_name) {
            skill.verified = true;
            Ok(())
        } else {
            Err("Skill not found".to_string())
        }
    }
}

// Define Job struct
#[derive(Clone, serde::Deserialize, serde::Serialize, CandidType)]
struct Job {
    title: String,
    description: String,
    requirements: Vec<String>,
    employer: String,
}

// Storage for managing users and jobs
struct Storage {
    users: HashMap<String, User>,
    jobs: HashMap<u64, Job>,
}

impl Storage {
    fn new() -> Self {
        Storage {
            users: HashMap::new(),
            jobs: HashMap::new(),
        }
    }

    fn add_user(&mut self, name: String) {
        self.users.insert(name.clone(), User::new(name));
    }

    fn add_skill_for_user(&mut self, user_name: String, skill: Skill) -> Result<(), String> {
        self.users.get_mut(&user_name).ok_or("User not found".to_string())
            .and_then(|user| {
                user.add_skill(skill);
                Ok(())
            })
    }

    fn update_skill_for_user(&mut self, user_name: String, skill_name: String, new_description: String, new_proficiency: f64) -> Result<(), String> {
        self.users.get_mut(&user_name).ok_or("User not found".to_string())
            .and_then(|user| user.update_skill(&skill_name, new_description, new_proficiency))
    }

    fn delete_skill_for_user(&mut self, user_name: String, skill_name: String) -> Result<(), String> {
        self.users.get_mut(&user_name).ok_or("User not found".to_string())
            .and_then(|user| user.delete_skill(&skill_name))
    }

    fn verify_skill_for_user(&mut self, user_name: String, skill_name: String) -> Result<(), String> {
        self.users.get_mut(&user_name).ok_or("User not found".to_string())
            .and_then(|user| user.verify_skill(&skill_name))
    }

    fn add_job(&mut self, job: Job) {
        let id = self.jobs.len() as u64;
        self.jobs.insert(id, job);
    }

    fn get_available_jobs(&self, user_name: String) -> Result<Vec<Job>, String> {
        let user = self.users.get(&user_name).ok_or("User not found".to_string())?;
        let mut available_jobs = Vec::new();
        for job in self.jobs.values() {
            if job.requirements.iter().all(|req| user.skills.iter().any(|skill| &skill.name == req)) {
                available_jobs.push(job.clone());
            }
        }
        if available_jobs.is_empty() {
            Err("No jobs available for the user's skills".to_string())
        } else {
            Ok(available_jobs)
        }
    }

    fn rate_user(&mut self, _user_name: String, _rating: f64) -> Result<(), String> {
        // Implement rating functionality
        Ok(())
    }
}

// Instantiate storage
thread_local! {
    static STORAGE: std::cell::RefCell<Storage> = std::cell::RefCell::new(Storage::new());
}

// Functions exposed to the Internet Computer
#[ic_cdk::update]
fn add_user(name: String) {
    STORAGE.with(|storage| {
        storage.borrow_mut().add_user(name);
    });
}

#[ic_cdk::update]
fn add_skill_for_user(user_name: String, skill: Skill) -> Result<(), String> {
    STORAGE.with(|storage| {
        storage.borrow_mut().add_skill_for_user(user_name, skill)
    })
}

#[ic_cdk::update]
fn update_skill_for_user(user_name: String, skill_name: String, new_description: String, new_proficiency: f64) -> Result<(), String> {
    STORAGE.with(|storage| {
        storage.borrow_mut().update_skill_for_user(user_name, skill_name, new_description, new_proficiency)
    })
}

#[ic_cdk::update]
fn delete_skill_for_user(user_name: String, skill_name: String) -> Result<(), String> {
    STORAGE.with(|storage| {
        storage.borrow_mut().delete_skill_for_user(user_name, skill_name)
    })
}

#[ic_cdk::update]
fn verify_skill_for_user(user_name: String, skill_name: String) -> Result<(), String> {
    STORAGE.with(|storage| {
        storage.borrow_mut().verify_skill_for_user(user_name, skill_name)
    })
}

#[ic_cdk::update]
fn add_job(job: Job) {
    STORAGE.with(|storage| {
        storage.borrow_mut().add_job(job);
    });
}

#[ic_cdk::query]
fn get_available_jobs(user_name: String) -> Result<Vec<Job>, String> {
    STORAGE.with(|storage| {
        storage.borrow().get_available_jobs(user_name)
    })
}

#[ic_cdk::update]
fn rate_user(user_name: String, rating: f64) -> Result<(), String> {
    STORAGE.with(|storage| {
        storage.borrow_mut().rate_user(user_name, rating)
    })
}

//Provides candid interface.
ic_cdk::export_candid!();