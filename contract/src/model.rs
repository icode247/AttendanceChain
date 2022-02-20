use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use near_sdk::{env, near_bindgen};

use crate::utils::{AccountId, Timestamp};

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Parent {
    user_id: AccountId,
    pub logged_minutes: Timestamp,
}

impl Parent {
    pub fn new() -> Self {
        Parent {
            user_id: env::signer_account_id(),
            logged_minutes: env::block_timestamp(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Student {
    user_id: AccountId,
    pub lessons: Vec<u32>,
    pub logged_minutes: Timestamp,
    student_name: String,
    school_name: String,
}

impl Student {
    pub fn new(student_name: String, school_name: String) -> Self {
        Student {
            // id: id,
            user_id: env::signer_account_id(),
            logged_minutes: env::block_timestamp(),
            lessons: vec![],
            student_name,
            school_name,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]

pub struct Lesson {
    creator: AccountId,
    pub id: u32,
    subject: String,
    pub task: String,
    pub is_approved: bool,
    pub is_completed: bool,
    created_at: Timestamp,
}

impl Lesson {
    pub fn new(id: u32, subject: String, task: String) -> Self {
        Lesson {
            creator: env::signer_account_id(),
            created_at: env::block_timestamp(),
            id: id,
            subject,
            task,
            is_approved: false,
            is_completed: false,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Attendance {
    pub lessons: Vec<u32>,
}

impl Attendance {
    pub fn new(lesson: u32) -> Self {
        Attendance {
            lessons: vec![lesson],
        }
    }
}
