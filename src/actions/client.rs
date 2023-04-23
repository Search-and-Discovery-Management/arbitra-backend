use elasticsearch::{
    Elasticsearch,
    http::{transport::{TransportBuilder,SingleNodeConnectionPool}},
};
use reqwest::{Url};

pub struct EClient {
    pub elastic: Elasticsearch
}

impl EClient {

    /// Creates a new instance of EClient
    /// 
    /// Connects to an instance of ElasticSearch server
    /// 
    /// Url: IP with Port (ex: "http://192.168.0.1:9200")
    pub fn new(url: &str) -> Self {
        
        let conn_url = Url::parse(url).unwrap();
        // Elasticsearch

        let conn_pool = SingleNodeConnectionPool::new(conn_url);
        let builder = TransportBuilder::new(conn_pool);
    
        let transport = builder.build().unwrap();
        
        Self{
            elastic: Elasticsearch::new(transport)
        }
    }
}