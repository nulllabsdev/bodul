

All Minisforum Models can be found on [support page](https://support.minisforum.com/pages/product-info)

Categorization:

```
    id            BIGINT / UUID     -- stable internal ID
    parent_id     BIGINT / UUID     -- current tree structure
    tree_code      CHAR(15)         -- generated hierarchy code
```

Example:

```
id:        4
name:      Electronics
parent_id: 0
tree_code: 001000000000000
```

```
id:        120
name:      Laptops
parent_id: 4
tree_code: 001002000000000
```

```
id:        58392
name:      Gaming Laptops
parent_id: 120
tree_code: 001002003000000
```

Add retailers like

https://www.refurbed.hr/ https://refurbished.com.hr/ Anker Ugreen