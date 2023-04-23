## POST /api/document/:app_id/:index
----
    Creates a new document

* **URL Params**

    **Required:**

    `app_id=[string]`

    `index=[string]`

* **Data Params**

    ```
    {
        <JSON Document>
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