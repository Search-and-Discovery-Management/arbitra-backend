# Testing Routes

## POST /api/another_test/test_data
----
    Inserts a test index into an application from the given link

* **URL Params**

    None

* **Data Params**

    ```
    {
        "app_id": <app_id>
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
