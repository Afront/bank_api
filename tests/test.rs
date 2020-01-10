#[cfg(test)]
mod tests {
	pub use bank_api::*;
	use chrono::prelude::*;

	#[test]
	fn bank_account_test() {
		let account = BankAccount::new("012-321".to_owned(), 0, vec![Transaction::new(Utc::now(),TransactionType::Deposit(123))]);
		assert_eq!(account.account_number(), "012-321".to_owned());
		assert_eq!(account.balance(), 0);
		assert_eq!(account.transaction_history()[0].time(), Utc::now());
		assert_eq!(account.transaction_history()[0].transaction_type(), TransactionType::Deposit(123));
	}

	#[test]
	fn bank_account_default_test() {
		let account: BankAccount = Default::default();
		assert_eq!(account.account_number(), "".to_owned());
		assert_eq!(account.balance(), 0);
		assert_eq!(account.transaction_history(), Vec::<Transaction>::new());
	}

	#[test]
	fn bank_account_overriding_default_test() {
		let mut account: BankAccount = Default::default();
		account.set_account_number("012-321".to_owned());
		assert_eq!(account.account_number(), "012-321".to_owned());
		assert_eq!(account.balance(), 0);
		assert_eq!(account.transaction_history(), Vec::<Transaction>::new());
	}

	#[test]
	fn transaction_history_test() {
		let deposit_transaction = Transaction::new(Utc::now(), TransactionType::Deposit(1));
		let transfer_transaction = Transaction::new(Utc::now(), TransactionType::Transfer(1, "123-321".to_owned()));
		let withdrawal_transaction = Transaction::new(Utc::now(), TransactionType::Withdraw(1));
		assert_eq!(deposit_transaction.time(), Utc::now());
		assert_eq!(deposit_transaction.transaction_type(), TransactionType::Deposit(1));
		assert_eq!(transfer_transaction.time(), Utc::now());
		assert_eq!(transfer_transaction.transaction_type(), TransactionType::Transfer(1, "123-321".to_owned()));
		assert_eq!(withdrawal_transaction.time(), Utc::now());
		assert_eq!(withdrawal_transaction.transaction_type(), TransactionType::Withdraw(1));
	}

	#[test]
	fn bank_account_functions_test() {
		let account = BankAccount::new("012-321".to_owned(), 0, vec![Transaction::new(Utc::now(),TransactionType::Deposit(123))]);
		assert_eq!(account.account_number(), "012-321".to_owned());
		assert_eq!(account.balance(), 0);
		assert_eq!(account.transaction_history()[0].time(), Utc::now());
		assert_eq!(account.transaction_history()[0].transaction_type(), TransactionType::Deposit(123));
	}
}