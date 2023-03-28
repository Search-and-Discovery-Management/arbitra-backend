# Application

## GET /api/apps
----
    Gets a list of applications

* **URL Params**

    None

* **Data Params**

    None

* **Headers**

    None

* **Success Response**
    * **Code:** 200
    
        **Content:**
        ```
        [
            {<data_object>},
            {<data_object>},
            {<data_object>}
        ]
        ```

## GET /api/app/:app_id
----
    Gets a specific application

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
        {<data_object>>}
        ```
* **Error Response**
    * **Code:** 404
        
        **Content:**
        ```
        {
            "error": "Application [id] not found"
        }
        ```

## POST /api/app
----
    Creates a new application

* **URL Params**

    None

* **Data Params**

    ```
    {
        "app_name": string
    }
    ```

* **Headers**

    None

* **Success Response**
    * **Code:** 201

* **Error Response**

    * **Code:** 409

## PUT /api/app
----
    Updates the mappings of an index

* **URL Params**

    None

* **Data Params**

    ```
    {
        "app_id": string,
        "app_name": string
    }
    ```

* **Headers**

    None

* **Success Response**
    * **Code:** 200

## DELETE /api/app/:app_id
----
    Deletes an application

* **URL Params**

    **Required:**

    `app_id=[string]`

* **Data Params**

    None

* **Headers**

    None

* **Success Response**
    * **Code:** 200

* **Error Response**
    * **Code:** 404

        ```
        {    
            "error": "Application [id] Not Found"
        }
        ```

# Index

## GET /api/index/:app_id
----
    Gets a list of index, specifying the name of the index returns only the specified index

* **URL Params**

    ***Required:***

    `app_id=[string]`

    ***Optional:*** 
    
    `index=[string]`


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
        
        **Content:**
        
        ```
        {
            "error": "Index [name] not found"
        }
        ```

        OR

        ```
        {
            "error": "Application [id] not found"
        }
        ```

## GET /api/index/mappings/:app_id/:index
----
    Gets the mappings of an index

* **URL Params**

    ***Required:*** 
    
    `app_id=[string]`
    `index=[string]`

* **Data Params**

    None

* **Headers**

    None

* **Success Response**
    * **Code:** 200
    
        **Content:**
        ```
            {
                "dynamic": "true"/"false"/"strict",
                "properties": {
                    <field_object>,
                    <field_object>,
                    <field_object>,
                    <field_object>,
                    ...
                }
                
            }
        ```
* **Error Response**
    * **Code:** 404
        
        **Content:**

        ```
        {
            "error": "Application [id] not found"
        }
        ```

        OR
        
        ```
        {    
            "error": "Index [name] not found"
        }
        ```

## POST /api/index
----
    Creates a new dynamic index

* **URL Params**

    None

* **Data Params**

    ```
    {
        "app_id": <app_id>,
        "index": string
    }
    ```

* **Headers**

    None

* **Success Response**
    * **Code:** 201

* **Error Response**

    * **Code:** 404

        **Content:**

            ```
            {    
                "error": "Application [id] Not Found"
            }
            ```

    * **Code:** 409

        **Content:**

            ```
            {    
                "error": "Failed to create new index, index [name] already exists"
            }
            ```

## PUT /api/index/mappings
----
    Updates the mappings of an index

* **URL Params**

    None

* **Data Params**

    ```
    {
        "app_id": string,
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

        **Content:**

        ```
        {    
            "error": "Bad Data Request"
        }
        ```

    * **Code:** 404
     
        **Content:**
        ```
        {    
            "error": "Index [name] not found"
        }
        ```
        OR

        ```
        {    
            "error": "Application [id] Not Found"
        }
        ```

## DELETE /api/index/:app_id/:index
----
    Deletes an index

* **URL Params**

    **Required:**

    `app_id=[string]`

    `index=[string]`

* **Data Params**

    None

* **Headers**

    None

* **Success Response**
    * **Code:** 200

* **Error Response**
    * **Code:** 404

        **Content:**
        ```
        {    
            "error": "Index [name] not found"
        }
        ```
        OR

        ```
        {    
            "error": "Application [id] Not Found"
        }
        ```

# Document

## GET /api/document/:app_id/:index/:document_id
----
    Returns a single document in an index
* **URL Params**

    ***Required:***

    `app_id=[string]`

    `index=[string]`
    
    `document_id=[string]`

* **Data Params**

    None

* **Headers**

    None

* **Success Response**
    * **Code:** 200

        **Content:**
        ```
        {<data object>}
        ```
* **Error Response**
    * **Code:** 404

        **Content:**

        ```
        {    
            "error": "Application [id] Not Found"
        }
        ```

        OR

        ```
        {    
            "error": "Index [name] not found"
        }
        ```

        OR

        ```
        {    
            "error": "Document ID [document_id] not found"
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
        "app_id": <app_id>
        "index": <index_name>,
        "search_term": string, (Optional)
        "search_in": <fields, comma separated>, (Optional)
        "return_fields": <fields, comma separated>, (Optional)
        "from": int, (Optional)
        "count": int, (Optional)
        "wildcards": bool (Optional)
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
                    {<data_object>},
                    {<data_object>},
                    {<data_object>}
                ],
            "match_type": string,
            "took": int,
            "total_data": int
        }
        ```
* **Error Response**
    * **Code:** 404

        ```
        {    
            "error": "Application [id] Not Found"
        }
        ```

        OR

        Content:
        ```
        {
            "error": "Index [name] not found"
        }
        ```

    * **Code:** 400

        Content:
        ```
        {
            "error": "Bad data request"
        }
        ```

## GET /api/search/:app_id/:index
----
    The same as post, searches an index for documents

* **URL Params**

    ***Required:*** 

    `app_id=[string]`

    `index=[string]`

    ***Optional:***

    `search_term=[string]`

    `search_in=[fields, comma separated]`

    `search_fields=[fields, comma separated]`

    `return_fields=[fields, comma separated]`

    `from=[int]`

    `count=[int]`

    `wildcards=[bool]`
* **Data Params**

    None

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
            "took": int,
            "total_data": int
        }
        ```
* **Error Response**
    * **Code:** 404

        Content:
        ```
        {    
            "error": "Application [id] Not Found"
        }
        ```

        OR

        ```
        {
            "error": "Index [name] not found"
        }
        ```

        OR

    * **Code:** 400

        Content:
        ```
        {
            "error": "Bad data request"
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
        "app_id": <app_id>,
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

        Content:
        ```
        {
            "error": "Bad data request"
        }
        ```

        OR

    * **Code:** 404

        Content:
        ```
        {    
            "error": "Application [id] Not Found"
        }
        ```

        OR

        ```
        {
            "error": "Index [name] not found"
        }
        ```

## PUT /api/document
----
    Updates a document

* **URL Params**
    
    None

* **Data Params**

    ```
    {
        "app_id": <app_id>,
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

        Content:
        ```
        {
            "error": "Bad data request"
        }
        ```

        OR

    * **Code:** 404

        Content:
        ```
        {    
            "error": "Application [id] Not Found"
        }
        ```

        OR

        ```
        {
            "error": "Index [name] not found"
        }
        ```
    
## DELETE /api/document/:app_id/:index/:document_id
----
    Deletes a document

* **URL Params**
    
    ***Required:***

    `app_id=[string]`

    `index=[string]`

    `document_id=[string]`

* **Data Params**

    None

* **Headers**

    None

* **Success Response**

    * **Code:** 200

* **Error Response**

    * **Code:** 404

        Content:
        ```
        {    
            "error": "Application [id] Not Found"
        }
        ```

        OR

        ```
        {
            "error": "Index [name] not found"
        }
        ```

        OR

        ```
        {
            "error": "Document ID [document_id] not found"
        }
        ```
