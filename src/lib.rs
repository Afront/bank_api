#![allow(dead_code)]

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
//	use serde_json::Result;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum TransactionType {
	Deposit(u64), //amount
	Transfer(u64, String), //amount, account_number
	Withdraw(u64), //amount
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct BankAccount {
	account_number: String,
	pub balance: u64,
	pub transaction_history: Vec<Transaction>,
}

impl BankAccount {
	pub fn new(account_number: u64, balance: u64, transaction_history: Vec<Transaction>) -> BankAccount {
		BankAccount {account_number: account_number.to_string(), balance: balance, transaction_history: transaction_history}
	}

	pub fn account_number(&self) -> String {
		self.account_number.clone()
	}
}

impl Default for BankAccount {
	fn default() -> BankAccount {
		BankAccount {account_number: "".to_owned(), balance: 0, transaction_history: Vec::<Transaction>::new()}
	}
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Transaction {
	time: DateTime<Utc>,
	transaction_type: TransactionType,
}

impl Transaction {
	pub fn new(time: DateTime<Utc>, transaction_type: TransactionType) -> Transaction {
		Transaction {time: time, transaction_type: transaction_type}
	}	

	pub fn time(&self) -> DateTime<Utc> {
		self.time
	}

	pub fn transaction_type(&self) ->  TransactionType { //Previously check_balance
		self.transaction_type.clone()
	}	
}