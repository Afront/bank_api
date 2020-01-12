#![forbid(unsafe_code)]

#[cfg(test)]
mod tests {
	pub use bank_api::*;
	use chrono::prelude::*;

	#[test]
	fn bank_account_test() {
		let account = BankAccount::new([1,2,3,4,5,6,7,8,1,2,3,4], 0, vec![Transaction::new(Utc::now(),TransactionType::Deposit(123))]);
		assert_eq!(account.account_number(), "AQ27 0000 1234 5678 1234".to_owned());
		assert_eq!(account.balance, 0);
		assert_eq!(account.transaction_history[0].time(), Utc::now());
		assert_eq!(account.transaction_history[0].transaction_type(), TransactionType::Deposit(123));
	}

	#[test]
	fn bank_account_default_test() {
		let account: BankAccount = Default::default();
		assert_eq!(account.account_number(), "AQ27 0000 0000 0000 0000".to_owned());
		assert_eq!(account.balance, 0);
		assert_eq!(account.transaction_history, Vec::<Transaction>::new());
	}

	#[test]
	fn bank_account_overriding_default_test() {
		let mut account: BankAccount = Default::default();
		account.balance += 1;
		assert_eq!(account.account_number(), "AQ27 0000 0000 0000 0000".to_owned());
		assert_eq!(account.balance, 1);
		assert_eq!(account.transaction_history, Vec::<Transaction>::new());
	}

	#[test]
	fn transaction_history_test() {
		let deposit_transaction = Transaction::new(Utc::now(), TransactionType::Deposit(1));
		let transfer_transaction = Transaction::new(Utc::now(), TransactionType::Transfer(1, 12345678));
		let withdrawal_transaction = Transaction::new(Utc::now(), TransactionType::Withdraw(1));
		assert_eq!(deposit_transaction.time(), Utc::now());
		assert_eq!(deposit_transaction.transaction_type(), TransactionType::Deposit(1));
		assert_eq!(transfer_transaction.time(), Utc::now());
		assert_eq!(transfer_transaction.transaction_type(), TransactionType::Transfer(1, 12345678));
		assert_eq!(withdrawal_transaction.time(), Utc::now());
		assert_eq!(withdrawal_transaction.transaction_type(), TransactionType::Withdraw(1));
	}

	#[test]
	fn bank_account_members_test() {
		let account = BankAccount::new([1,2,3,4,5,6,7,8,1,2,3,4], 0, vec![Transaction::new(Utc::now(),TransactionType::Deposit(123))]);
		assert_eq!(*account.account_number(), "AQ27 0000 1234 5678 1234".to_owned());
		assert_eq!(account.balance, 0);
		assert_eq!(account.transaction_history[0].time(), Utc::now());
		assert_eq!(account.transaction_history[0].transaction_type(), TransactionType::Deposit(123));
	}

	#[test]
	fn bank_account_methods_test() {
		let mut account = BankAccount::new([1,2,3,4,5,6,7,8,1,2,3,4], 0, vec![Transaction::new(Utc::now(),TransactionType::Deposit(123))]);
		let mut other_account: BankAccount = Default::default();
		account.deposit(100);
		account.withdraw(10);
		account.transfer(5, &mut other_account);
	}


}