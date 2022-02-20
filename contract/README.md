my-project Smart Contract
==================

A [smart contract] written in [Rust] for an app initialized with [create-near-app]


Quick Start
===========

Before you compile this code, you will need to install Rust with [correct target]


Exploring The Code
==================
1. Clone the repository
2. Run `yarn` to install all the packages
3. The main smart contract code lives in `src/lib.rs`. You can compile it with
   the `./compile` script.
4. Use the command below to run the contract locally `yarn dev`


  [smart contract]: https://docs.near.org/docs/develop/contracts/overview
  [Rust]: https://www.rust-lang.org/
  [create-near-app]: https://github.com/near/create-near-app
  [correct target]: https://github.com/near/near-sdk-rs#pre-requisites
  [cargo]: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html


Test the contract
==================

 1. set_parent: This functions adds a parent to the smart contract. eg: `near call dev-1645307407162-70213718739518 set_parent  --account-id dev-1645307407162-70213718739518`

 2. set_student: This functions add a student to the smart contract eg: `near call dev-1645307407162-70213718739518 set_student '{"student_name":"","school_name":""}'  --account-id dev-1645307407162-70213718739518`

 3. set_lesson: This function adds a new lesson to the blockchain, each of the lessons stores the ID of the parents that created them. eg: `dev-1645307407162-70213718739518 set_lesson '{"subject":"","task":""}'  --account-id dev-1645307407162-70213718739518`

 4. asign_lesseon: This functions allows a parent to assign lessons to their children. Eg: `dev-1645307407162-70213718739518 assign_lesson '{"student_id":"","lesson_id":""}'  --account-id dev-1645307407162-70213718739518`

 5. complete_lesson: This function allows a child to complete a lesson. Eg. `dev-1645307407162-70213718739518 complete_lesson '{"id":"","task":""}'  --account-id dev-1645307407162-70213718739518`

 6. approve_lesson: This function allows a parent to approved a lesson. Eg. `dev-1645307407162-70213718739518 approve_lesson '{"id":""}'  --account-id dev-1645307407162-70213718739518`