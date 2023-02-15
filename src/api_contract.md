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
            {
                "docs.count": "0",
                "docs.deleted": "0",
                "health": "green",
                "index": "test_index",
                "pri": "3",
                "pri.store.size": "675b",
                "rep": "0",
                "status": "open",
                "store.size": "675b",
                "uuid": "qyX3NoR8SXOPkA0EoiDWRg"
            }
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
    Updates the mappings of an index

* **URL Params**

    None

* **Data Params**

    ```
    {
        "index": string,
        "mappings": value
    }
    ```

* **Headers**

    None

* **Success Response**
    * **Code:** 200

` `
* **Error Response**

    * **Code:** 400
    * **Code:** 404

