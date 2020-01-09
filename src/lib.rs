#[cfg(test)]
mod tests {
	use chrono::prelude::*;
	use serde::{Deserialize, Serialize};
//	use serde_json::Result;

	#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
	enum TransactionType {
		Deposit(u64), //amount
		Transfer(u64, String), //amount, account_number
		Withdraw(u64), //amount
	}

	#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
	struct BankAccount {
		account_number: String,
		balance: u64,
		transaction_history: Vec<Transaction>,
	}

	impl BankAccount {
		pub fn new(account_number: String, balance: u64, transaction_history: Vec<Transaction>) -> BankAccount {
			BankAccount {account_number: account_number, balance: balance, transaction_history: transaction_history}
		}

		pub fn get_account_number(&self) -> String {
			self.account_number.clone()
		}

		pub fn get_balance(&self) ->  u64 { //Previously check_balance
			self.balance
		}	
		
		pub fn get_transaction_history(&self) ->  Vec<Transaction> { 
			self.transaction_history.clone()
		}
	}

	#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
	struct Transaction {
		time: DateTime<Utc>,
		transaction_type: TransactionType,
	}

	impl Transaction {
		fn new(time: DateTime<Utc>, transaction_type: TransactionType) -> Transaction {
			Transaction {time: time, transaction_type: transaction_type}
		}
	}
}