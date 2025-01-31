# DataSculptor

A tool to compile and analyze data from [Eriks-Data-Collector](https://github.com/ErikSeewald/Eriks-Data-Collector) (private).


Can be used for other data as long as it is in the correct data format.

## Data format

A json file of the type:\
{\
  &emsp;&emsp;"YYYY-MM-DD": \
  &emsp;&emsp;{\
    &emsp;&emsp;&emsp;&emsp;"some key": "some value.",\
    &emsp;&emsp;&emsp;&emsp;"another key": "**Only string values are allowed.**",\
    &emsp;&emsp;&emsp;&emsp;[...]\
  &emsp;&emsp;},

  &emsp;&emsp;"YYYY-MM-DD": \
  &emsp;&emsp;{\
      &emsp;&emsp;&emsp;&emsp;"some key": "some value",\
      &emsp;&emsp;&emsp;&emsp;"another key": "another value"\
      &emsp;&emsp;&emsp;&emsp;"yet another key": "**Different date entries can have different sets of keys.**",\
      &emsp;&emsp;&emsp;&emsp;[...]\
 &emsp; &emsp;},

  &emsp;&emsp;[...]

}

## Filter syntax
The [list view](/data_sculptor/src/gui/views/list) allows you to set different filters for how you want to display the data.

- If an expression does not match any valid syntax (even if the difference is whitespaces), the gui will not show any errors. It will simply not let you submit the filter.

#### Filter types
- *Date*: If the date does not match the date filter, the entire day will be filtered out.
- *Key*: Any keys that match at least one of the key filters will be shown. The rest are filtered out. This way you can choose which key-value pairs to show.
- *Value*: If a value does not match the value filter, the entire day will be filtered out. This is why commands like 'contains' are not available for value filters and are instead replaced by key-specific commands like 'kv-contains'. More about that in the command description.

#### Logical Operators
Logical operators are used to connect different filter expressions together. You can use parentheses () to define operator precedence.
| Operator     | Description                                                   | Syntax                        
|-------------|---------------------------------------------------------------|-------------------------------|
| **not**     | Adding 'not' before another expression inverts it | `not <expression>`              |
| **and**     | True if both expressions are true | `<expression1> and <expression2>`              |
| **or**     | True if at least one expression is true | `<expression1> or <expression2>`              |
| **xor**     | True if exactly one expression is true | `<expression1> xor <expression2>`              |
| **nand**     | True if at least one of the expressions is false | `<expression1> nand <expression2>`              |
| **nor**     | True if neither expression is true | `<expression1> nor <expression2>`              |
| **xnor**     | True if both expressions have the same boolean value | `<expression1> xnor <expression2>`              |

- *Do not negate logical operators using the keyword 'not'*. Just use the provided negations (e.g., 'nand', 'nor', etc.). Due to the way the postfix parser works, `{a} not and {b}` is equivalent to `not {a} and {b}`. (However, `not ({a} and {b})` is still equivalent to `{a} nand {b}`)

#### Filter Commands
Filter commands are specific ways for you to filter the data. They are wrapped in curly brackets {} and can be connected with logical operators. If you do not use any logical operators, the curly brackets {} can be omitted.
| Command     | Description                                                   | Syntax                        | Example                                                  | Implemented by filter types |
|-------------|---------------------------------------------------------------|-------------------------------|----------------------------------------------------------|-----------------------------|
| **contains**| Checks if the value contains the given string                 | `{contains "<keyword>"}`       | `{contains "2"}` is true if '2' is found in the value.     | Date, Key                   |
| **kv-contains** | 'Key Value Contains' checks if the value corresponding to the given key contains the given string | `{kv-contains "<key>" "<keyword>}"` | `{contains "food" "tomato"}` is true if the value for 'food' contains 'tomato'. | Value             |
| **numop** | Tries to parse the value to a number and then compares it to the number you give it. Defaults to false if parsing fails. | `{numop "'>' or '<'" "<number>}"` | `{numop ">" "8.3"}` is true if the value is greater than 8.3 | Date, Key             |
| **kv-numop** | 'Key Value Numop' does the same as 'numop' but only for the value corresponding to the given key | `{kv-numop "<key>" "'>' or '<'" "<number>}"` | `{kv-numop "speed" "<" "20"}` is true if the value for 'speed' is less than 20. | Value             |
| **date** | Lets you set a date specific condition | `{date <'before' or 'after'> "<YYYY-MM-DD>"}"` | `{date before "2024-02-01"}` is true if the date is before february 2024 | Date             |


#### Expression examples:
- `not ({contains "2"} and ({contains "3"} or {contains "4"}))` is true for "25" and "3" but false for "23" and "24".
