# Index

## GET /api/index?:index
----
    Gets a list of index, specifying the name of the index returns only the specified index

* **URL Params**

    ***Optional:*** `index=[string]`

* **Data Params**

    None

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


## POST /api/index
----
    Creates a new dynamic index

* **URL Params**

    None

* **Data Params**

    ```
    {
        "index": string
    }
    ```

* **Headers**

    None

* **Success Response**
    * **Code:** 201

* **Error Response**
    * **Code:** 409

## PUT /api/mappings
----
    Updates the mappings of an index

* **URL Params**

    None

* **Data Params**

    ```
    {
        "index": string,
        "mappings": <json_object>
    }
    ```

* **Headers**

    None

* **Success Response**
    * **Code:** 200

* **Error Response**

    * **Code:** 400

        OR

    * **Code:** 404

# Document

## GET /api/document/:index/:document_id
----
    Returns a single document in an index
* **URL Params**

    ***Required:***

    `index=[string]`
    
    `document_id=[string]`

* **Data Params**

    None

* **Headers**

    None

* **Success Response**
    * **Code:** 200

        Content:
        ```
        {<data object>}
        ```
* **Error Response**
    * **Code:** 404

        Content:
        ```
        {
            "message": "not found"
        }
        ```

## POST /api/search
----
    Searches an index for documents

* **URL Params**

    None

* **Data Params**

    ```
    {
        "index": <index_name>,
        "search_term": string, (Optional)
        "search_in": <fields, comma separated>, (Optional)
        "return_fields": <fields, comma separated>, (Optional)
        "from": int, (Optional)
        "count": int (Optional)
    }
    ```

* **Headers**

    None

* **Success Response**
    * **Code:** 200

        Content:
        ```
        {
            "data" = 
                [
                    {<document_object>},
                    {<document_object>},
                    {<document_object>}
                ],
            "match_type": string,
            "status": <status_code>,
            "took": int,
            "total_data": int

        }

        ```
* **Error Response**
    * **Code:** 404

        Content:
        ```
        {
            "message": "not found"
        }
        ```

        OR

    * **Code:** 400

        Content:
        ```
        {
            "message": "bad request"
        }
        ```

## POST /api/document
----
    Creates a new document

* **URL Params**

    None

* **Data Params**

    ```
    {
        "index": <index_name>,
        "dynamic_mode": <modes: "true", "false", "strict">, (Optional)
        "data": <json_object>
    }
    ```
* **Headers**

    None

* **Success Response**

    * **Code:** 201

* **Error Response**
    * **Code:** 400

        OR

    * **Code:** 404

## PUT /api/document
----
    Updates a document

* **URL Params**
    
    None

* **Data Params**

    ```
    {
        "index": <index_name>,
        "dynamic_mode": <modes: "true", "false", "strict"> (Optional),
        "data": <json_object>
    }
    ```
* **Headers**

    None

* **Success Response**

    * **Code:** 200

* **Error Response**
    * **Code:** 400

        OR

    * **Code:** 404
    
## DELETE /api/document/:index/:document_id
----
    Deletes a document

* **URL Params**
    
    ***Required:***

    `index=[string]`

    `document_id=[string]`

* **Data Params**

    None

* **Headers**

    None

* **Success Response**

    * **Code:** 200

    Content:
    ```
    {
        "message": "successfully updated"
    }
    ```

* **Error Response**

    * **Code:** 404

    Content:
    ```
    {
        "message": "not found"
    }
    ```
