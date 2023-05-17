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

## DELETE /api/another_test/delete/delete_everything
----
    Deletes all apps and indexes

* **URL Params**

    None

* **Data Params**

    None

* **Headers**

    None

* **Response**
    * **Code:** 200
    

## GET /api/index/list/:app_id
----
    Gets a list of index that exists in an app

* **URL Params**

    ***Required:***

    `app_id=[string]`


* **Data Params**

    None

* **Headers**

    None

* **Success Response**
    * **Code:** 200
    
        **Content:**
        ```
        [
            "index_name_1",
            "index_name_2",
            "index_name_3",
            ...
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