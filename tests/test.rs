#[cfg(test)]
mod tests {
	pub use bank_api::*;
	use chrono::prelude::*;

	#[test]
	fn bank_account_test() {
		let account = BankAccount::new("012-321".to_owned(), 0, vec![Transaction::new(Utc::now(),TransactionType::Deposit(123))]);
		assert_eq!(account.get_account_number(), "012-321".to_owned());
		assert_eq!(account.get_balance(), 0);
		assert_eq!(account.get_transaction_history()[0].get_time(), Utc::now());
		assert_eq!(account.get_transaction_history()[0].get_transaction_type(), TransactionType::Deposit(123));
	}

	#[test]
	fn bank_account_default_test() {
		let account: BankAccount = Default::default();
		assert_eq!(account.get_account_number(), "".to_owned());
		assert_eq!(account.get_balance(), 0);
		assert_eq!(account.get_transaction_history(), Vec::<Transaction>::new());
	}

/**
	#[test]
	fn bank_account_overriding_default_test() {
		let account = BankAccount {account_number: "012-321".to_owned(), ..Default::default()};
		assert_eq!(account.get_account_number(), "012-321".to_owned());
		assert_eq!(account.get_balance(), 0);
		assert_eq!(account.get_transaction_history(), Vec::<Transaction>::new());
	}
**/


	#[test]
	fn transaction_history_test() {
		let deposit_transaction = Transaction::new(Utc::now(), TransactionType::Deposit(1));
		let transfer_transaction = Transaction::new(Utc::now(), TransactionType::Transfer(1, "123-321".to_owned()));
		let withdrawal_transaction = Transaction::new(Utc::now(), TransactionType::Withdraw(1));
		assert_eq!(deposit_transaction.get_time(), Utc::now());
		assert_eq!(deposit_transaction.get_transaction_type(), TransactionType::Deposit(1));
		assert_eq!(transfer_transaction.get_time(), Utc::now());
		assert_eq!(transfer_transaction.get_transaction_type(), TransactionType::Transfer(1, "123-321".to_owned()));
		assert_eq!(withdrawal_transaction.get_time(), Utc::now());
		assert_eq!(withdrawal_transaction.get_transaction_type(), TransactionType::Withdraw(1));
	}

	#[test]
	fn bank_account_functions_test() {
		let account = BankAccount::new("012-321".to_owned(), 0, vec![Transaction::new(Utc::now(),TransactionType::Deposit(123))]);
		assert_eq!(account.get_account_number(), "012-321".to_owned());
		assert_eq!(account.get_balance(), 0);
		assert_eq!(account.get_transaction_history()[0].get_time(), Utc::now());
		assert_eq!(account.get_transaction_history()[0].get_transaction_type(), TransactionType::Deposit(123));
	}
}