use crate::models::*;
use chrono::{DateTime, Duration, Local};
use reqwest::{header, header::HeaderMap, Client};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{fs, io};

type ApiResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct YnabApi {
    base_url: String,
    //headers = {"Authorization": f"Bearer {TOKEN}"}
    //token = token
    cache: HashMap<String, CacheEntry>,
    pub cache_hit: u32,
    pub cache_miss: u32,
    client: Client,
    token: String,
    cache_file: String,
    pub force_refresh: bool,
}

impl YnabApi {
    pub fn new(token: &str, cache_file: &str) -> Self {
        let cache = fs::File::open(cache_file)
            .map(|f| serde_json::from_reader(io::BufReader::new(f)).unwrap_or_default())
            .unwrap_or_default();

        Self {
            base_url: String::from("https://api.youneedabudget.com/v1"),
            cache,
            cache_hit: 0,
            cache_miss: 0,
            client: Client::new(),
            token: token.to_string(),
            cache_file: cache_file.to_string(),
            force_refresh: false,
        }
    }

    async fn get(&mut self, endpoint: &str) -> ApiResult<String> {
        if let Some(cache_entry) = self.cache.get(endpoint) {
            if Local::now() - cache_entry.datetime < Duration::hours(1) && !self.force_refresh {
                self.cache_hit += 1;
                return Ok(cache_entry.response_json.clone());
            }
        }
        self.cache_miss += 1;
        let mut headers = HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            format!("Bearer {}", self.token)
                .parse()
                .expect("Created auth header"),
        );
        let resp = self
            .client
            .get(format!("{}{}", self.base_url, endpoint))
            .headers(headers)
            .send()
            .await?
            .text()
            .await?;

        self.cache.insert(
            endpoint.to_string(),
            CacheEntry {
                datetime: Local::now(),
                response_json: resp.clone(),
            },
        );
        Ok(resp)
    }

    /// TODO: Implement include accounts feature
    pub async fn list_budgets(&mut self, _include_accounts: bool) -> ApiResult<Data<BudgetSummaryResponse>> {
        let endp = "/budgets";
        Ok(serde_json::from_str(&self.get(endp).await?)?)
    }

    /// TODO
    pub async fn get_budget(&mut self, _budget_id: &str, _last_knowledge_of_server: Option<i32>) -> ApiResult<Data<()>> {
        todo!("GET /budgets/{_budget_id}")
    }

    /// TODO
    pub async fn budget_settings(&mut self, _budget_id: &str) -> ApiResult<Data<()>> {
        todo!("GET /budgets/{_budget_id}/settings")
    }

    /// TODO
    pub async fn list_accounts(&mut self, _budget_id: &str, _last_knowledge_of_server: Option<i32>) -> ApiResult<Data<()>> {
        todo!("GET /budgets/{_budget_id}/accounts")
    }

    /// TODO
    pub async fn create_account(&mut self, _budget_id: &str, _name: &str, _type: &str, _balance: i64) -> ApiResult<Data<()>> {
        todo!("POST /budgets/{_budget_id}/accounts")
    }

    /// TODO
    pub async fn get_account(&mut self, _budget_id: &str, _account_id: &str) -> ApiResult<Data<()>> {
        todo!("GET /budgets/{_budget_id}/accounts/{_account_id}")
    }

    pub async fn list_categories(&mut self, budget_id: &str) -> ApiResult<Data<CategoriesResponse>> {
        let endp = &format!("/budgets/{budget_id}/categories");
        Ok(serde_json::from_str(&self.get(endp).await?)?)
    }

    pub async fn get_category_transactions(
        &mut self,
        budget_id: &str,
        category_id: &str,
    ) -> ApiResult<Data<HybridTransactionsResponse>> {
        let endp = &format!("/budgets/{budget_id}/categories/{category_id}/transactions");
        Ok(serde_json::from_str(&self.get(endp).await?)?)
    }
}

impl Drop for YnabApi {
    fn drop(&mut self) {
        let f = std::fs::File::create(&self.cache_file).expect("Failed to create cache file");
        let w = std::io::BufWriter::new(f);
        serde_json::to_writer(w, &self.cache).expect("Writing cache to disk failed");
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct CacheEntry {
    datetime: DateTime<Local>,
    response_json: String,
}
