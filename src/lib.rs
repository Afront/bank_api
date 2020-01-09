#[cfg(test)]
mod tests {
	use chrono::prelude::*;
	use serde::{Deserialize, Serialize};
//	use serde_json::Result;

	#[derive(Debug, PartialEq)]
	enum Transaction {
		Deposit(u64), //amount
		Transfer(u64, String), //amount, account_number
		Withdraw(u64), //amount
	}

	#[derive(Debug, Deserialize, Serialize)]
	struct BankAccount {
		account_number: String,
		balance: u64,
		transaction_history: Vec<String>,
	}

	#[derive(Debug)]
	struct TransactionHistory {
		time: DateTime<Utc>,
		transaction: Transaction,
	}

	#[test]
	fn bank_account_test() {
		let account = BankAccount {account_number: "012-321".to_owned(), balance: 0, transaction_history: vec!["hi".to_owned()]};
		assert_eq!(account.account_number, "012-321".to_owned());
		assert_eq!(account.balance, 0);
		assert_eq!(account.transaction_history[0], "hi");
	}

	#[test]
	fn transaction_history_test() {
		let deposit_transaction = TransactionHistory {time: Utc::now(), transaction: Transaction::Deposit(1)};
		let transfer_transaction = TransactionHistory {time: Utc::now(), transaction: Transaction::Transfer(1, "123-321".to_owned())};
		let withdrawal_transaction = TransactionHistory {time: Utc::now(), transaction: Transaction::Withdraw(1)};
		assert_eq!(deposit_transaction.time, Utc::now());
		assert_eq!(deposit_transaction.transaction, Transaction::Deposit(1));
		assert_eq!(transfer_transaction.time, Utc::now());
		assert_eq!(transfer_transaction.transaction, Transaction::Transfer(1, "123-321".to_owned()));
		assert_eq!(withdrawal_transaction.time, Utc::now());
		assert_eq!(withdrawal_transaction.transaction, Transaction::Withdraw(1));
	}

}