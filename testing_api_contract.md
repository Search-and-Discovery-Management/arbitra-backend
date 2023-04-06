# Testing Routes

## POST /api/another_test/test_data/:app_id
----
    Inserts a test index into an application from the given link

* **URL Params**

    **Required:**

    `app_id=[string]`

* **Data Params**

    ```
    {
        "index": <index_name>, (Optional)
        "shards": int, (Optional)
        "replicas": int, (Optional)
        "link": string (Optional)
    }
    ```

* **Headers**

    None

* **Success Response**
    * **Code:** 200
    
        **Content:**
        ```
        [
            {<index_object>},
            {<index_object>},
            {<index_object>}
        ]
        ```
* **Error Response**
    * **Code:** 404
        
        **Content:**

        ```
        {
            "error": "Application [id] not found"
        }
        ```


## POST /api/another_test/bulk_add_data
----
    Inserts a vector of json data into an index

* **URL Params**

    **Required:**

    `app_id=[string]`

    `index=[string]`

* **Data Params**

    ```
    {
        "data": [
            <data_object>, 
            <data_object>, 
            <data_object>
            ]
    }
    ```

* **Headers**

    None

* **Success Response**
    * **Code:** 200
    
        **Content:**
        ```
        [
            {<index_object>},
            {<index_object>},
            {<index_object>}
        ]
        ```
* **Error Response**
    * **Code:** 404
        
        **Content:**

        ```
        {
            "error": "Application [id] not found"
        }
        ```
