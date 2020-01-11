#![forbid(unsafe_code)]
#![allow(dead_code)]

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
//	use serde_json::Result;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum TransactionType {
	Deposit(u64), //amount
	Transfer(u64, u64), //amount, account_number
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
		let bban = &account_number.to_string();
		if bban.len() != 12 { //note: not .chars().count() as it's O(N) and it is converted from an unsigned integer, so len() should be better
			panic!("The length of the account number should be exactly 12 digits long!");
		}
		let iban = format!{"AQ{:02} 0000 {} {} {}", 98 - (account_number * 10000 + 1026) * 100 % 97, &bban[0..4], &bban[4..8], &bban[8..12]};

		BankAccount {account_number: iban, balance: balance, transaction_history: transaction_history}
	}

	pub fn account_number(&self) -> &String {
		&self.account_number
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