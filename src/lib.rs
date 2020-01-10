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

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BankAccount {
	account_number: String,
	balance: u64,
	transaction_history: Vec<Transaction>,
}

impl BankAccount {
	pub fn new(account_number: String, balance: u64, transaction_history: Vec<Transaction>) -> BankAccount {
		BankAccount {account_number: account_number, balance: balance, transaction_history: transaction_history}
	}

	pub fn account_number(&self) -> String {
		self.account_number.clone()
	}

	pub fn balance(&self) ->  u64 { //Previously check_balance
		self.balance
	}	
	
	pub fn transaction_history(&self) ->  Vec<Transaction> { 
		self.transaction_history.clone()
	}

	pub fn set_account_number(&mut self, account_number: String) {
		self.account_number = account_number;
	}

	pub fn set_balance(&mut self, balance: u64) { //Previously check_balance
		self.balance = balance;
	}	
	
	pub fn set_transaction_history(&mut self, transaction_history: Vec<Transaction>) { 
		self.transaction_history = transaction_history;
	}}

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