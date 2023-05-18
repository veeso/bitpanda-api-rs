use std::str::FromStr;

use crate::ApiError;

/// Transaction status
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Deserialize)]
pub enum TransactionStatus {
    Canceled,
    Finished,
    OpenInvitation,
    Pending,
    Processing,
    Unconfirmed,
    UnconfirmedTransactionOut,
}

impl ToString for TransactionStatus {
    fn to_string(&self) -> String {
        match self {
            TransactionStatus::Canceled => "canceled",
            TransactionStatus::Finished => "finished",
            TransactionStatus::OpenInvitation => "open_invitation",
            TransactionStatus::Pending => "pending",
            TransactionStatus::Processing => "processing",
            TransactionStatus::Unconfirmed => "unconfirmed",
            TransactionStatus::UnconfirmedTransactionOut => "unconfirmed_transaction_out",
        }
        .to_string()
    }
}

impl FromStr for TransactionStatus {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "canceled" => Ok(Self::Canceled),
            "finished" => Ok(Self::Finished),
            "open_invitation" => Ok(Self::OpenInvitation),
            "pending" => Ok(Self::Pending),
            "processing" => Ok(Self::Processing),
            "unconfirmed" => Ok(Self::Unconfirmed),
            "unconfirmed_transaction_out" => Ok(Self::UnconfirmedTransactionOut),
            _ => Err(ApiError::UnexpectedValue(s.to_string())),
        }
    }
}

/// Transaction type
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Deserialize)]
pub enum TransactionType {
    Buy,
    Deposit,
    Ico,
    Refund,
    Sell,
    Transfer,
    Withdrawal,
}

impl FromStr for TransactionType {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "buy" => Ok(Self::Buy),
            "deposit" => Ok(Self::Deposit),
            "ico" => Ok(Self::Ico),
            "refund" => Ok(Self::Refund),
            "sell" => Ok(Self::Sell),
            "transfer" => Ok(Self::Transfer),
            "withdrawal" => Ok(Self::Withdrawal),
            _ => Err(ApiError::UnexpectedValue(s.to_string())),
        }
    }
}

impl ToString for TransactionType {
    fn to_string(&self) -> String {
        match self {
            TransactionType::Buy => "buy",
            TransactionType::Sell => "sell",
            TransactionType::Deposit => "deposit",
            TransactionType::Ico => "ico",
            TransactionType::Refund => "ico",
            TransactionType::Transfer => "transfer",
            TransactionType::Withdrawal => "withdrawal",
        }
        .to_string()
    }
}

/// Transaction "direction"
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Deserialize)]
pub enum InOrOut {
    Incoming,
    Outgoing,
}

impl FromStr for InOrOut {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "incoming" => Ok(Self::Incoming),
            "outgoing" => Ok(Self::Outgoing),
            _ => Err(ApiError::UnexpectedValue(s.to_string())),
        }
    }
}
