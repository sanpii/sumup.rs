#[derive(Clone, Debug)]
pub struct Transactions<'a> {
    api: &'a crate::Api,
    access_token: &'a crate::AccessToken,
}

impl<'a> Transactions<'a> {
    pub fn new(api: &'a crate::Api, access_token: &'a crate::AccessToken) -> Self {
        Self { api, access_token }
    }

    pub fn find_by_id(&self, id: &str) -> crate::Result<crate::Transaction> {
        self.api.transactions_get(id, self.access_token)
    }

    pub fn find_by_internal_id(&self, internal_id: &str) -> crate::Result<crate::Transaction> {
        self.api
            .transactions_get_by_internal_id(internal_id, self.access_token)
    }

    pub fn find_by_code(&self, code: &str) -> crate::Result<crate::Transaction> {
        self.api.transactions_get_by_code(code, self.access_token)
    }

    pub fn history(&self, filter: &Filter) -> crate::Result<Vec<crate::Transaction>> {
        self.api.transactions_history(filter, self.access_token)
    }

    pub fn full_refund(&self, id: u32) -> crate::Result {
        let payload = ureq::json!({});

        self.api.transactions_refund(id, payload, self.access_token)
    }

    pub fn refund(&self, id: u32, amount: f32) -> crate::Result {
        let payload = ureq::json!({
            "amount": amount,
        });

        self.api.transactions_refund(id, payload, self.access_token)
    }

    pub fn receipt(&self, id: u32, merchant_id: u32) -> crate::Result<crate::Receipt> {
        self.api
            .transactions_get_receipt(id, merchant_id, self.access_token)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Filter {
    start_date: String,
    end_date: String,
    order: Option<String>,
    limit: Option<u32>,
    user_id: Option<u32>,
    users: Vec<u32>,
    statuses: Vec<crate::Status>,
    payment_types: Vec<crate::PaymentType>,
    types: Vec<crate::Type>,
    changes_since: Option<String>,
    newest_time: Option<String>,
    newest_ref: Option<String>,
    oldest_time: Option<String>,
    oldest_ref: Option<String>,
}

impl ToString for Filter {
    fn to_string(&self) -> String {
        let mut v = Vec::new();

        v.push(format!(
            "start_date={}&end_date={}&order={}",
            self.start_date,
            self.end_date,
            self.order.as_deref().unwrap_or("asc")
        ));
        v.push(format!("limit={}", self.limit.unwrap_or(10)));

        if let Some(user_id) = &self.user_id {
            v.push(format!("user_id={user_id}"));
        }

        for user in &self.users {
            v.push(format!("user={user}"));
        }

        for statuse in &self.statuses {
            v.push(format!("statuse={}", statuse.to_string()));
        }

        for payment_type in &self.payment_types {
            v.push(format!("payment_type={}", payment_type.to_string()));
        }

        for ty in &self.types {
            v.push(format!("type={}", ty.to_string()));
        }

        if let Some(changes_since) = &self.changes_since {
            v.push(format!("changes_since={changes_since}"));
        }

        if let Some(newest_time) = &self.newest_time {
            v.push(format!("newest_time={newest_time}"));
        }

        if let Some(newest_ref) = &self.newest_ref {
            v.push(format!("newest_ref={newest_ref}"));
        }

        if let Some(oldest_time) = &self.oldest_time {
            v.push(format!("oldest_time={oldest_time}"));
        }

        if let Some(oldest_ref) = &self.oldest_ref {
            v.push(format!("oldest_ref={oldest_ref}"));
        }

        v.push("format=json".to_string());

        v.join("&")
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn find_by_id() -> crate::Result {
        let api = crate::test::api()?;

        api.transactions().find_by_id("1")?;

        Ok(())
    }

    #[test]
    fn find_by_code() -> crate::Result {
        let api = crate::test::api()?;

        api.transactions().find_by_code("1234")?;

        Ok(())
    }

    #[test]
    fn find_by_internal_id() -> crate::Result {
        let api = crate::test::api()?;

        api.transactions().find_by_internal_id("1234")?;

        Ok(())
    }

    #[test]
    fn history() -> crate::Result {
        let api = crate::test::api()?;

        let filter = crate::services::transactions::Filter {
            start_date: "2021-01-01".to_string(),
            end_date: "2021-12-31".to_string(),

            ..Default::default()
        };

        let history = api.transactions().history(&filter)?;
        if history.is_empty() {
            log::warn!("Empty response");
        }

        Ok(())
    }

    #[test]
    fn full_refund() -> crate::Result {
        let api = crate::test::api()?;

        api.transactions().full_refund(1)?;

        Ok(())
    }

    #[test]
    fn refund() -> crate::Result {
        let api = crate::test::api()?;

        api.transactions().refund(1, 1.2)?;

        Ok(())
    }

    #[test]
    fn receipt() -> crate::Result {
        let api = crate::test::api()?;

        api.transactions().receipt(1, 1)?;

        Ok(())
    }
}
