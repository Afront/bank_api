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

	#[test]
	fn bank_account_test() {
		let account = BankAccount {account_number: "012-321".to_owned(), balance: 0, transaction_history: vec![Transaction::new(Utc::now(),TransactionType::Deposit(123))]};
		assert_eq!(account.account_number, "012-321".to_owned());
		assert_eq!(account.balance, 0);
		assert_eq!(account.transaction_history[0].time, Utc::now());
		assert_eq!(account.transaction_history[0].transaction_type, TransactionType::Deposit(123));
	}

	#[test]
	fn bank_account_new_test() {
		let account = BankAccount::new("012-321".to_owned(), 0, vec![Transaction::new(Utc::now(),TransactionType::Deposit(123))]);
		assert_eq!(account.account_number, "012-321".to_owned());
		assert_eq!(account.balance, 0);
		assert_eq!(account.transaction_history[0].time, Utc::now());
		assert_eq!(account.transaction_history[0].transaction_type, TransactionType::Deposit(123));
	}

	#[test]
	fn bank_account_default_test() {
		let account: BankAccount = Default::default();
		assert_eq!(account.account_number, "".to_owned());
		assert_eq!(account.balance, 0);
		assert_eq!(account.transaction_history, Vec::<Transaction>::new());
	}

	#[test]
	fn bank_account_overriding_default_test() {
		let account = BankAccount {account_number: "012-321".to_owned(), ..Default::default()};
		assert_eq!(account.account_number, "012-321".to_owned());
		assert_eq!(account.balance, 0);
		assert_eq!(account.transaction_history, Vec::<Transaction>::new());
	}

	#[test]
	fn transaction_history_test() {
		let deposit_transaction = Transaction {time: Utc::now(), transaction_type: TransactionType::Deposit(1)};
		let transfer_transaction = Transaction {time: Utc::now(), transaction_type: TransactionType::Transfer(1, "123-321".to_owned())};
		let withdrawal_transaction = Transaction {time: Utc::now(), transaction_type: TransactionType::Withdraw(1)};
		assert_eq!(deposit_transaction.time, Utc::now());
		assert_eq!(deposit_transaction.transaction_type, TransactionType::Deposit(1));
		assert_eq!(transfer_transaction.time, Utc::now());
		assert_eq!(transfer_transaction.transaction_type, TransactionType::Transfer(1, "123-321".to_owned()));
		assert_eq!(withdrawal_transaction.time, Utc::now());
		assert_eq!(withdrawal_transaction.transaction_type, TransactionType::Withdraw(1));
	}

	#[test]
	fn bank_account_functions_test() {
		let account = BankAccount::new("012-321".to_owned(), 0, vec![Transaction::new(Utc::now(),TransactionType::Deposit(123))]);
		assert_eq!(account.get_account_number(), "012-321".to_owned());
		assert_eq!(account.get_balance(), 0);
		assert_eq!(account.get_transaction_history()[0].time, Utc::now());
		assert_eq!(account.get_transaction_history()[0].transaction_type, TransactionType::Deposit(123));
	}
}