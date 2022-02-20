mod model;
mod utils;

use crate::{
    model::{Attendance, Lesson, Parent, Student},
    utils::AccountId,
};

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use near_sdk::{env, near_bindgen, PromiseIndex};
near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(Clone, Default, BorshDeserialize, BorshSerialize)]

pub struct Contract {
    owner: AccountId,
    lessons: Vec<Lesson>,
    parents: Vec<Parent>,
    attendance: Vec<Attendance>,
    students: Vec<Student>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn init(owner: AccountId) -> Self {
        let lessons: Vec<Lesson> = Vec::new();
        let parents: Vec<Parent> = Vec::new();
        let students: Vec<Student> = Vec::new();
        let attendance: Vec<Attendance> = Vec::new();

        Contract {
            owner,
            lessons,
            parents,
            students,
            attendance,
        }
    }

    pub fn set_parent(&mut self) {
        self.parents.push(Parent::new());
        env::log("Added a new user to contract".as_bytes());
    }

    pub fn set_student(&mut self, student_name: String, school_name: String) {
        self.students.push(Student::new(student_name, school_name));
        env::log("Added a new student to contract".as_bytes());
    }

    pub fn set_lesson(&mut self, subject: String, task: String) {
        let id = self.lessons.len() as u32;
        self.lessons.push(Lesson::new(id, subject, task));
        env::log("Added a new lesson to contract".as_bytes());
    }

    pub fn assign_lesson(&mut self, student_id: usize, lesson_id: usize) {
        let lesson: &Lesson = self.lessons.get_mut(lesson_id).unwrap();
        let student: &mut Student = self.students.get_mut(student_id).unwrap();
        student.lessons.push(lesson.id);
        env::log("You have assigned a lesson to a student".as_bytes());
    }

    pub fn complete_lesson(&mut self, id: usize, task: String) {
        //Get the lesson id
        let lesson: &mut Lesson = self.lessons.get_mut(id).unwrap();
        //Modify the blockchain
        lesson.task = task;
        lesson.is_completed = true;
        env::log("Approved a new lesson on the contract".as_bytes());
    }

    pub fn approve_lesson(&mut self, id: usize) {
        //Get the lesson id
        let lesson: &mut Lesson = self.lessons.get_mut(id).unwrap();
        //Modify the blockchain
        lesson.is_approved = true;
        if self.attendance.len() > 0 {
            self.attendance[0].lessons.push(lesson.id);
        } else {
            self.attendance.push(Attendance::new(lesson.id));
        }
        env::log("Approved a new lesson on the contract".as_bytes());
    }

    pub fn get_lesson(&self, id: usize) -> Option<&Lesson> {
        self.lessons.get(id)
    }

    pub fn list_lessons(&self) -> Vec<Lesson>{
        let lessons = &self.lessons;
        return lessons.to_vec();
    }

    pub fn list_parents(&self) -> Vec<Parent> {
        let parents = &self.parents;
        return parents.to_vec();
    }

    pub fn list_students(&self) -> Vec<Student> {
        let students = &self.students;
        return students.to_vec();
    }

    pub fn list_attendance(&self) -> Vec<Attendance> {
        let attendance = &self.attendance;
        return attendance.to_vec();
    }
}
