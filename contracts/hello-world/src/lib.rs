#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol, Vec, Map};

#[contract]
pub struct TodoContract;

#[contractimpl]
impl TodoContract {

    // Add a task
    pub fn add_task(env: Env, user: Symbol, task: Symbol) {
        let key = symbol_short!("TASKS");

        let mut tasks: Map<Symbol, Vec<Symbol>> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        let mut user_tasks = tasks.get(user.clone()).unwrap_or(Vec::new(&env));

        user_tasks.push_back(task);
        tasks.set(user, user_tasks);

        env.storage().instance().set(&key, &tasks);
    }

    // View tasks
    pub fn get_tasks(env: Env, user: Symbol) -> Vec<Symbol> {
        let key = symbol_short!("TASKS");

        let tasks: Map<Symbol, Vec<Symbol>> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        tasks.get(user).unwrap_or(Vec::new(&env))
    }

    // Remove a task (by index)
    pub fn remove_task(env: Env, user: Symbol, index: u32) {
        let key = symbol_short!("TASKS");

        let mut tasks: Map<Symbol, Vec<Symbol>> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        let mut user_tasks = tasks.get(user.clone()).unwrap_or(Vec::new(&env));

        if index < user_tasks.len() {
            user_tasks.remove(index);
        }

        tasks.set(user, user_tasks);
        env.storage().instance().set(&key, &tasks);
    }
}