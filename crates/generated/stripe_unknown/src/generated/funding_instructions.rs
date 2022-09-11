// ======================================
// This file was automatically generated.
// ======================================

use async_stripe_common::resources::Currency;
use async_stripe_core::params::Object;
use serde::{Deserialize, Serialize};

use crate::resources::FundingInstructionsBankTransferFinancialAddress;

/// The resource representing a Stripe "CustomerBalanceFundingInstructionsCustomerBalanceFundingInstructions".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FundingInstructions {
    pub bank_transfer: FundingInstructionsBankTransfer,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// The `funding_type` of the returned instructions.
    pub funding_type: FundingInstructionsFundingType,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
}

impl Object for FundingInstructions {
    type Id = ();
    fn id(&self) -> Self::Id {}
    fn object(&self) -> &'static str {
        "funding_instructions"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct FundingInstructionsBankTransfer {
    /// The country of the bank account to fund.
    pub country: String,

    /// A list of financial addresses that can be used to fund a particular balance.
    pub financial_addresses: Vec<FundingInstructionsBankTransferFinancialAddress>,

    /// The bank_transfer type.
    #[serde(rename = "type")]
    pub type_: FundingInstructionsBankTransferType,
}

/// An enum representing the possible values of an `FundingInstructionsBankTransfer`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FundingInstructionsBankTransferType {
    EuBankTransfer,
    JpBankTransfer,
}

impl FundingInstructionsBankTransferType {
    pub fn as_str(self) -> &'static str {
        match self {
            FundingInstructionsBankTransferType::EuBankTransfer => "eu_bank_transfer",
            FundingInstructionsBankTransferType::JpBankTransfer => "jp_bank_transfer",
        }
    }
}

impl AsRef<str> for FundingInstructionsBankTransferType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FundingInstructionsBankTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for FundingInstructionsBankTransferType {
    fn default() -> Self {
        Self::EuBankTransfer
    }
}

/// An enum representing the possible values of an `FundingInstructions`'s `funding_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FundingInstructionsFundingType {
    BankTransfer,
}

impl FundingInstructionsFundingType {
    pub fn as_str(self) -> &'static str {
        match self {
            FundingInstructionsFundingType::BankTransfer => "bank_transfer",
        }
    }
}

impl AsRef<str> for FundingInstructionsFundingType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FundingInstructionsFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for FundingInstructionsFundingType {
    fn default() -> Self {
        Self::BankTransfer
    }
}