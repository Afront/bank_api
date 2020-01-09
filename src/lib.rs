#[cfg(test)]
mod tests {
	use chrono::prelude::*;
	use serde::{Deserialize, Serialize};
//	use serde_json::Result;

	#[derive(Debug, PartialEq)]
	enum TransactionType {
		Deposit(u64), //amount
		Transfer(u64, String), //amount, account_number
		Withdraw(u64), //amount
	}

	#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
	struct BankAccount {
		account_number: String,
		balance: u64,
		transaction_history: Vec<String>,
	}

	#[derive(Debug)]
	struct Transaction {
		time: DateTime<Utc>,
		transaction: TransactionType,
	}

	impl BankAccount {
		fn new(account_number: String, balance: u64, transaction_history: Vec<String>) -> BankAccount {
			BankAccount {account_number: account_number, balance: balance, transaction_history: transaction_history}
		}
	}

	#[test]
	fn bank_account_test() {
		let account = BankAccount {account_number: "012-321".to_owned(), balance: 0, transaction_history: vec!["hi".to_owned()]};
		assert_eq!(account.account_number, "012-321".to_owned());
		assert_eq!(account.balance, 0);
		assert_eq!(account.transaction_history[0], "hi");
	}

	#[test]
	fn bank_account_new_test() {
		let account = BankAccount::new("012-321".to_owned(), 0, vec!["hi".to_owned()]);
		assert_eq!(account.account_number, "012-321".to_owned());
		assert_eq!(account.balance, 0);
		assert_eq!(account.transaction_history[0], "hi");
	}

	#[test]
	fn bank_account_default_test() {
		let account: BankAccount = Default::default();
		assert_eq!(account.account_number, "".to_owned());
		assert_eq!(account.balance, 0);
		assert_eq!(account.transaction_history, Vec::<String>::new());
	}

	#[test]
	fn bank_account_overriding_default_test() {
		let account = BankAccount {account_number: "012-321".to_owned(), ..Default::default()};
		assert_eq!(account.account_number, "012-321".to_owned());
		assert_eq!(account.balance, 0);
		assert_eq!(account.transaction_history, Vec::<String>::new());
	}

	//Change name
	#[test]
	fn transaction_history_test() {
		let deposit_transaction = Transaction {time: Utc::now(), transaction: TransactionType::Deposit(1)};
		let transfer_transaction = Transaction {time: Utc::now(), transaction: TransactionType::Transfer(1, "123-321".to_owned())};
		let withdrawal_transaction = Transaction {time: Utc::now(), transaction: TransactionType::Withdraw(1)};
		assert_eq!(deposit_transaction.time, Utc::now());
		assert_eq!(deposit_transaction.transaction, TransactionType::Deposit(1));
		assert_eq!(transfer_transaction.time, Utc::now());
		assert_eq!(transfer_transaction.transaction, TransactionType::Transfer(1, "123-321".to_owned()));
		assert_eq!(withdrawal_transaction.time, Utc::now());
		assert_eq!(withdrawal_transaction.transaction, TransactionType::Withdraw(1));
	}

}