#[cfg(test)]
mod tests {
	use chrono::prelude::*;
	use serde::{Deserialize, Serialize};
//	use serde_json::Result;

	#[derive(Debug, Deserialize, Serialize)]
	struct BankAccount {
		account_number: String,
		balance: u64,
		transaction_history: Vec<String>,
	}

	#[derive(Debug, Deserialize, Serialize)]
	struct TransactionHistory {
		time: DateTime<Utc>,
		transaction: String,
	}

	#[test]
	fn bank_account_test() {
		let account = BankAccount {account_number: "012-321".to_owned(), balance: 0, transaction_history: vec!["hi".to_owned()]};
		assert_eq!(account.account_number , "012-321".to_owned());
		assert_eq!(account.balance , 0);
		assert_eq!(account.transaction_history[0] , "hi");
	}

	#[test]
	fn transaction_history_test() {
		let transaction = TransactionHistory {time: Utc::now(), transaction: "hi".to_owned()};
		assert_eq!(transaction.time, Utc::now());
		assert_eq!(transaction.transaction, "hi");
	}

}