use anyhow::{bail, Error as AnyError, Result};
use chrono::prelude::*;
use serde::de::{self, Deserializer, Unexpected};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub trait InputRecord {
    fn get_date(&self) -> &String;

    fn get_amount(&self) -> f64;
}

#[derive(Debug, Deserialize)]
pub struct Subscan {
    #[serde(rename = "Event Index")]
    pub event_id: String,

    #[serde(rename = "Date")]
    pub date: String,

    #[serde(rename = "Block")]
    pub block: u64,

    #[serde(rename = "Extrinsic Index")]
    pub extrinsic: String,

    #[serde(rename = "Value")]
    pub amount: f64,

    #[serde(rename = "Action")]
    pub action: String,
}

impl InputRecord for Subscan {
    fn get_date(&self) -> &String {
        &self.date
    }

    fn get_amount(&self) -> f64 {
        self.amount
    }
}

#[derive(Debug, Deserialize)]
pub struct Kraken {
    pub txid: String,
    pub refid: String,

    #[serde(rename = "time")]
    pub date: String,

    #[serde(rename = "type")]
    pub action: String,

    pub aclass: String,
    pub asset: String,
    pub amount: f64,
    pub fee: f64,
}

impl InputRecord for Kraken {
    fn get_date(&self) -> &String {
        &self.date
    }

    fn get_amount(&self) -> f64 {
        self.amount
    }
}

#[derive(Debug, Deserialize)]
pub struct StakeTax {
    pub timestamp: String,
    pub tx_type: String,
    #[serde(deserialize_with = "bool_from_string")]
    pub taxable: bool,
    pub received_amount: Option<f64>,
    pub received_currency: String,
    pub sent_amount: Option<f64>,
    pub sent_currency: String,
    pub fee: Option<f64>,
    pub fee_currency: String,
    pub comment: String,
    #[serde(rename = "txid")]
    pub tx_id: String,
    pub url: String,
    pub exchange: String,
    pub wallet_address: String,
}

impl InputRecord for StakeTax {
    fn get_date(&self) -> &String {
        &self.timestamp
    }

    fn get_amount(&self) -> f64 {
        match self.received_amount {
            Some(amount) => amount,
            None => 0f64,
        }
    }
}

#[derive(Debug, Deserialize)]
pub enum InputFormat {
    Subscan,
    Kraken,
    StakeTax,
}

#[derive(Debug)]
pub enum OutputFormat {
    BitcoinTax,
    CoinTracking,
}

#[derive(Debug)]
pub enum Quarter {
    Q1,
    Q2,
    Q3,
    Q4,
    ALL,
}

impl FromStr for Quarter {
    type Err = AnyError;

    fn from_str(s: &str) -> Result<Self> {
        match &s.to_lowercase()[..] {
            "q1" | "1" => Ok(Quarter::Q1),
            "q2" | "2" => Ok(Quarter::Q2),
            "q3" | "3" => Ok(Quarter::Q3),
            "q4" | "4" => Ok(Quarter::Q4),
            "all" => Ok(Quarter::ALL),
            _ => bail!("Invalid quarter!"),
        }
    }
}

#[derive(Debug, Serialize)]
pub enum OutputRecord {
    BT(BitcoinTax),
    CT(CoinTracking),
}

#[derive(Debug, Serialize)]
pub struct BitcoinTax {
    date: NaiveDateTime,
    action: String,
    account: String,
    symbol: Coin,
    volume: f64,
}

impl BitcoinTax {
    pub fn create(date: NaiveDateTime, volume: f64, symbol: Coin) -> Self {
        let coin: String = symbol.into();
        let account = format!("{} STAKING", coin);

        Self {
            date,
            action: "INCOME".into(),
            account,
            symbol,
            volume,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CoinTracking {
    #[serde(rename = "Type")]
    tx_type: String,
    #[serde(rename = "Buy Amount")]
    buy_amount: f64,
    #[serde(rename = "Buy Currency")]
    buy_currency: String,
    #[serde(rename = "Sell Amount")]
    sell_amount: f64,
    #[serde(rename = "Sell Currency")]
    sell_currency: Currency,
    #[serde(rename = "Fee")]
    fee: f64,
    #[serde(rename = "Fee Currency")]
    fee_currency: Currency,
    #[serde(rename = "Exchange")]
    exchange: String,
    #[serde(rename = "Trade-Group")]
    trade_group: String,
    #[serde(rename = "Comment")]
    comment: String,
    #[serde(rename = "Date")]
    date: NaiveDateTime,
    #[serde(rename = "Tx-ID")]
    tx_id: String,
    #[serde(rename = "Buy Value in Account Currency")]
    buy_value: f64,
}

impl CoinTracking {
    pub fn create(
        buy_amount: f64,
        buy_currency: String,
        trade_group: String,
        comment: String,
        date: NaiveDateTime,
    ) -> Self {
        Self {
            tx_type: "Income".into(),
            buy_amount,
            buy_currency,
            sell_amount: 0.0,
            sell_currency: Currency::USD,
            fee: 0.0,
            fee_currency: Currency::USD,
            exchange: "".into(),
            trade_group,
            comment,
            date,
            tx_id: "".into(),
            buy_value: 0.0,
        }
    }
}

#[derive(Debug, Serialize)]
enum Currency {
    USD,
    GBP,
    EUR,
}

impl FromStr for Currency {
    type Err = AnyError;

    fn from_str(s: &str) -> Result<Self> {
        match &s.to_lowercase()[..] {
            "usd" => Ok(Currency::USD),
            "gbp" => Ok(Currency::GBP),
            "eur" => Ok(Currency::EUR),
            _ => bail!("Invalid currency type!"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum Coin {
    DOT,
    KSM,
    ATOM,
    ETH,
    SOL,
    KAVA,
    ADA,
    XTZ,
}

impl FromStr for Coin {
    type Err = AnyError;

    fn from_str(s: &str) -> Result<Coin, AnyError> {
        match &s.to_lowercase()[..] {
            "dot" | "dot.s" => Ok(Coin::DOT),
            "ksm" | "ksm.s" => Ok(Coin::KSM),
            "atom" | "atom.s" => Ok(Coin::ATOM),
            "eth" | "eth.s" | "eth2" | "eth2.s" => Ok(Coin::ETH),
            "sol" | "sol.s" => Ok(Coin::SOL),
            "kava" | "kava.s" => Ok(Coin::KAVA),
            "ada" | "ada.s" => Ok(Coin::ADA),
            "xtz" | "xtz.s" => Ok(Coin::XTZ),
            _ => panic!("Invalid coin type!"),
        }
    }
}

impl From<String> for Coin {
    fn from(s: String) -> Self {
        match &s.to_lowercase()[..] {
            "dot" | "dot.s" => Coin::DOT,
            "ksm" | "ksm.s" => Coin::KSM,
            "atom" | "atom.s" => Coin::ATOM,
            "eth" | "eth.s" | "eth2" | "eth2.s" => Coin::ETH,
            "sol" | "sol.s" => Coin::SOL,
            "kava" | "kava.s" => Coin::KAVA,
            "ada" | "ada.s" => Coin::ADA,
            "xtz" | "xtz.s" => Coin::XTZ,
            _ => panic!("Invalid coin type!"),
        }
    }
}

impl Into<String> for Coin {
    fn into(self) -> String {
        match self {
            Coin::DOT => String::from("DOT2"),
            Coin::KSM => String::from("KSM"),
            Coin::ATOM => String::from("ATOM"),
            Coin::ETH => String::from("ETH"),
            Coin::SOL => String::from("SOL"),
            Coin::KAVA => String::from("KAVA"),
            Coin::ADA => String::from("ADA"),
            Coin::XTZ => String::from("XTZ"),
        }
    }
}

/// Deserialize bool from String with custom value mapping
fn bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match &String::deserialize(deserializer)?.to_lowercase()[..] {
        "true" => Ok(true),
        "" | "false" => Ok(false),
        other => Err(de::Error::invalid_value(
            Unexpected::Str(other),
            &"true or false",
        )),
    }
}
