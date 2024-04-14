# DataSculptor

A tool to compile and analyze the data produced by [Eriks-Data-Collector](https://github.com/ErikSeewald/Eriks-Data-Collector).


Can be used for other data as long as it is in the following format:

## Data format

A json file of the type:\
"\
{
  >"YYYY-MM-DD": \
  {\
    "some key": "some value.",\
    "another key": "**Only string values are allowed.**",\
    [...]\
  },

  >"YYYY-MM-DD": \
  {\
      "some key": "some value",\
      "another key": "another value"\
      "yet another key": "**As you can see, different date entries can have different sets of keys.**",\
      [...]\
  },

  >[...]

}\
"
