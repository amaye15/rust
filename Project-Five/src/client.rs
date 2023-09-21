use reqwest;

pub struct Service {
    base_url: String,
    auth_token: Option<String>,
}

impl Service {
    pub fn new(base_url: &str) -> Self {
        Service {
            base_url: base_url.to_string(),
            auth_token: None,
        }
    }

    pub fn set_auth_token(&mut self, token: &str) {
        self.auth_token = Some(token.to_string());
    }

    pub fn index(&self, name: &str) -> Index {
        Index {
            service: self.clone(),
            name: name.to_string(),
        }
    }
}

pub struct Index {
    service: Service,
    name: String,
}

impl Index {
    pub fn submit(&self, event: &str, sourcetype: &str) -> Result<(), reqwest::Error> {
        let client = reqwest::blocking::Client::new();

        // Construct the endpoint URL
        let endpoint = format!("{}/receivers/simple", self.service.base_url);

        // Add authorization header if auth_token is available
        let mut headers = reqwest::header::HeaderMap::new();
        if let Some(token) = &self.service.auth_token {
            headers.insert("Authorization", token.parse().unwrap());
        }

        // Define the parameters for the request
        let params = [("sourcetype", sourcetype)];

        // Make the HTTP request
        let response = client.post(&endpoint)
            .headers(headers)
            .form(&params)
            .send()?;

        // Check if the request was successful
        if response.status().is_success() {
            Ok(())
        } else {
            Err(reqwest::Error::new(response.status(), response.text()?))
        }
    }
}
