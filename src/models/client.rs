// use crate::models;

// struct Client {
//     elastic_search: elasticsearch::ElasticSearch
// }


// impl Client{

//     pub fn new() -> Self{
//         let url = Url::parse("http://127.0.0.1:9201").unwrap();

//         let conn_pool = SingleNodeConnectionPool::new(url);
//         let builder = TransportBuilder::new(conn_pool);

//         let transport = builder.build()?;
//         return Self{
//             elastic_search: Elasticsearch::new(transport)
//         }
//     }

//     fn create_index(client: &Elasticsearch, index: &str) -> Result<(), Error> {
//         let exists = self
//             .indices()
//             .exists(IndicesExistsParts::Index(&[index]))
//             .send()
//             .await?;
    
//         if exists.status_code() == StatusCode::NOT_FOUND {
//             let response = client
//                 .indices()
//                 .create(IndicesCreateParts::Index(index))
//                 .body(json!(
//                     {
//                       "mappings": { 	
//                         "dynamic":"true"
//                       },
//                       "settings": {
//                         "index.number_of_shards": 3,
//                         "index.number_of_replicas": 0,
//                       }
//                     }
//                 ))
//                 .send()
//                 .await?;
    
//             if !response.status_code().is_success() {
//                 println!("Error found: {:#?}", response);
//             }
//         }
    
//         Ok(())
//     }





// }