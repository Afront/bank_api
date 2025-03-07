#![forbid(unsafe_code)]
#![allow(dead_code)]

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
//	use serde_json::Result;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum TransactionType {
	Deposit(u64), //amount
	Transfer(u64, u64), //amount, account_number
	Withdraw(u64), //amount
}


#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BankAccount {
	account_number: [u8;12],
	pub balance: u64,
	pub transaction_history: Vec<Transaction>,
}


impl BankAccount {
	pub fn new(account_number: [u8;12], balance: u64, transaction_history: Vec<Transaction>) -> BankAccount {
		BankAccount {account_number: account_number, balance: balance, transaction_history: transaction_history}
	}

	pub fn account_number(&self) -> String {
		let mut bban = String::new();
		let mut actual_number: u64 = 0;

		for digit in &self.account_number {
			bban += &digit.to_string();
			actual_number += (*digit) as u64;
			actual_number *= 10;
		}

		format!{"AQ{:02} 0000 {} {} {}", 98 - (actual_number * 10000 + 1026) * 100 % 97, &bban[0..4], &bban[4..8], &bban[8..12]}
	}

	pub fn deposit(&mut self, deposit_amount: u64) {
		self.balance += deposit_amount;
		self.transaction_history.push(Transaction::new(Utc::now(), TransactionType::Deposit(deposit_amount)));
	}

	pub fn transfer(&mut self, transfer_amount: u64, other: &mut BankAccount) {
		if self.balance < transfer_amount {
			panic!("Not enough money!"); //Will change to Result
			//self.transaction_history.push(Transaction.new(Utc::now(), TransactionType::Error("Withdrawal", withdrawal_amount, Error::NotEnoughMoney)));
			//return Err(Error::NotEnoughMoney);
		}

		self.balance -= transfer_amount;
		other.balance += transfer_amount;
		//self.transaction_history.push(Transaction::new(Utc::now(), TransactionType::Transfer(transfer_amount, -self.account_number_as_uint())));
		//other.transaction_history.push(Transaction::new(Utc::now(), TransactionType::Transfer(transfer_amount, self.account_number_as_uint())));
	}

	pub fn withdraw(&mut self, withdrawal_amount: u64) {
		if self.balance < withdrawal_amount {
			panic!("Not enough money!"); //Will change to Result
			//self.transaction_history.push(Transaction.new(Utc::now(), TransactionType::Error("Withdrawal", withdrawal_amount, Error::NotEnoughMoney)));
			//return Err(Error::NotEnoughMoney);
		}
		self.balance -= withdrawal_amount;
		self.transaction_history.push(Transaction::new(Utc::now(), TransactionType::Withdraw(withdrawal_amount)));
	}
}

impl Default for BankAccount {
	fn default() -> BankAccount {
		BankAccount {account_number: [0,0,0,0,0,0,0,0,0,0,0,0], balance: 0, transaction_history: Vec::<Transaction>::new()}
	}
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
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
		self.transaction_type
	}	
}