#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, vec, Address, Env, IntoVal, 
    TryFromVal, Val, Vec,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Datakey {
    Account(Address),
    Bank,
    Admin,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Account {
    pub account_name: Address,
    pub balance: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Bank {
   pub accounts: Vec<Account>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Kind {
    Instance,
    Permanent,
    Temporary,
}

#[contract]
pub struct BankContract;

pub trait BankContractTrait {
    fn init(e: Env, admin: Address) -> bool;
    fn create_account(e: Env, person: Address, amount: i128) -> Account;
    fn accounts(env: Env, person: Address) -> Vec<Account>;
    fn myaccount_balance(e: Env, person: Address) -> i128;
    fn close_account(e:Env, person: Address, admin: Address)-> bool;
    fn deposit(env:Env, person: Address, amount: i128)-> Account;
    fn widraw(env:Env, person: Address, amount: i128)-> Account;
}

#[contractimpl]
impl BankContractTrait for BankContract {
    fn init(e: Env, admin: Address) -> bool {
        assert!(!e.storage().persistent().has(&Datakey::Admin));

        let bank = Bank { accounts: vec![&e] };

        storage_p(e.clone(), admin, Kind::Permanent, Datakey::Admin);

        storage_p(e, bank, Kind::Permanent, Datakey::Bank)
    }

    fn create_account(e: Env, person: Address, amount: i128) -> Account {
        person.require_auth();
        let acc = Account {
            account_name: person.clone(),
            balance: amount,
        };

        let mut bank: Bank =
            storage_g(e.clone(), Kind::Permanent, Datakey::Bank).expect("cound not found a bank");
        bank.accounts.push_back(acc.clone());

        storage_p(
            e.clone(),
            acc.clone(),
            Kind::Permanent,
            Datakey::Account(person),
        );
        storage_p(e, bank, Kind::Permanent, Datakey::Bank);

        acc
    }


    fn deposit(e:Env, person: Address, amount: i128)-> Account{
        person.require_auth();

        let  mut bank: Bank = storage_g(e.clone(), Kind::Permanent, Datakey::Bank).expect("cound not found a bank");
        let mut accs:Vec<Account> =bank.accounts;
        
        let index:usize = accs.clone().iter().position(|x| x.account_name == person).expect("Coundnt get position");
        let mut account:Account = storage_g(e.clone(), Kind::Permanent, Datakey::Account(person.clone())).expect("Account Error");
        account.balance = account.balance + amount;
         
        accs.remove(index as u32);
        accs.push_back(account.clone());

        bank.accounts = accs;

        storage_p(e.clone(), bank, Kind::Permanent, Datakey::Bank);
        storage_p(e, account.clone(),Kind::Permanent, Datakey::Account(person));

        account


    }



    fn widraw(e:Env, person: Address, amount: i128)-> Account{
        person.require_auth();

        let  mut bank: Bank = storage_g(e.clone(), Kind::Permanent, Datakey::Bank).expect("cound not found a bank");
        let mut accs:Vec<Account> =bank.accounts;
        
        let index:usize = accs.clone().iter().position(|x| x.account_name == person).expect("Coundnt get position");
        let mut account:Account = storage_g(e.clone(), Kind::Permanent, Datakey::Account(person.clone())).expect("Account Error");
        
        assert!(account.balance > amount);
        account.balance = account.balance - amount;
         
        accs.remove(index as u32);
        accs.push_back(account.clone());

        bank.accounts = accs;

        storage_p(e.clone(), bank, Kind::Permanent, Datakey::Bank);
        storage_p(e, account.clone(),Kind::Permanent, Datakey::Account(person));

        account


    }

    fn accounts(e: Env, person: Address) -> Vec<Account> {
        assert_eq!(get_admin(e.clone()), person);
        let bank: Bank =
            storage_g(e, Kind::Permanent, Datakey::Bank).expect("cound not found a bank");
        bank.accounts
    }

    fn myaccount_balance(e: Env, person: Address) -> i128 {
        person.require_auth();
        let account: Account = e
            .storage()
            .persistent()
            .get(&Datakey::Account(person))
            .unwrap();
        account.balance
    }


    fn close_account(e:Env, person: Address, admin: Address)-> bool{
        admin.require_auth();

        let mut bank: Bank = storage_g(e.clone(), Kind::Permanent, Datakey::Bank).expect("cound not found a bank");
        let mut accs:Vec<Account> =bank.accounts;

         let index =  accs.clone().iter().position(|x| x.account_name == person).expect("Account not found");


      let done = match accs.remove(index as u32) {
        Some( _) => {
            bank.accounts = accs;
            storage_p(e, bank, Kind::Permanent, Datakey::Bank);
            true
        },
        None => false
          
      };  


       

        done
    }
}

fn storage_g<T: IntoVal<Env, Val> + TryFromVal<Env, Val>>(
    env: Env,
    kind: Kind,
    key: Datakey,
) -> Option<T> {
    let done: Option<T> = match kind {
        Kind::Instance => env.storage().instance().get(&key),

        Kind::Permanent => env.storage().persistent().get(&key),

        Kind::Temporary => env.storage().temporary().get(&key),
    };

    done
}

fn storage_p<T: IntoVal<Env, Val>>(env: Env, value: T, kind: Kind, key: Datakey) -> bool {
    let done: bool = match kind {
        Kind::Instance => {
            env.storage().instance().set(&key, &value);
            true
        }

        Kind::Permanent => {
            env.storage().persistent().set(&key, &value);
            true
        }

        Kind::Temporary => {
            env.storage().temporary().set(&key, &value);
            true
        }
    };

    done
}

fn get_admin(env: Env) -> Address {
    storage_g(env, Kind::Permanent, Datakey::Admin).unwrap()
}

mod test;
