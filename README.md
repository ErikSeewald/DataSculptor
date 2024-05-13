# DataSculptor

A tool to compile and analyze data from [Eriks-Data-Collector](https://github.com/ErikSeewald/Eriks-Data-Collector).


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

#### Filter types
- *Date*: If the date does not match the date filter, the entire day will be filtered out.
- *Key*: Any keys that match at least one of the key filters will be shown. The rest are filtered out. This way you can choose which key-value pairs to show.
- *Value*: If a value does not match the value filter, the entire day will be filtered out. This is why commands like 'contains' are not available for value filters and are instead replaced by key-specific commands like 'kv-contains'. More about that in the command description.

#### Commands
If a command does not match any command syntax (even if the difference is whitespaces), the gui will not show any errors. It will simple not let you submit the filter.

| Keyword     | Description                                                   | Syntax                        | Example                                                  | Implemented by filter types |
|-------------|---------------------------------------------------------------|-------------------------------|----------------------------------------------------------|-----------------------------|
| **not**     | Adding 'not' before your command inverts the result of a filter | `not <command>`              | `not contains "2"` is true if '2' is not found.          | Date, Key, Value            |
| **contains**| Checks if the value contains the given string                 | `contains "<keyword>"`       | `contains "2"` is true if '2' is found in the value.     | Date, Key                   |
| **kv-contains** | 'Key Value Contains' checks if the value corresponding to the given key contains the given string | `kv-contains "<key>" "<keyword>"` | `contains "food" "tomato"` is true if the value for 'food' contains 'tomato'. | Value             |
