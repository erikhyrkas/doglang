# DataFrame

A dataframe is a concept used in a number of languages and provides the ability to query and manipulate data.

The DataFrame trait returns DataFrameRow traits, which is backed by a structure holding a list of elements each of
the DataFrameValue trait, which in turn supports the data type coercion as necessary.

More details to follow.

# SQL
Dog supports SQL natively, which means you can run queries that are syntax checked at compile time.

Dog SQL supports structured and semi-structured data in a familiar and comfortable way.

SQL can be run within backticks (`) and can use local variables (lists, maps, arrays, etc.) to query against. 

Example:
```
    let result: DataFrame = `select * from y limit 100;` 
```

Internally, it is creating a DataFrame from the sql and returning it, so the functionality is equivalent. 

