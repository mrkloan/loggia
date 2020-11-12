# Loggia
[![Pipeline](https://github.com/MrKloan/loggia/workflows/CI/badge.svg)](https://github.com/MrKloan/loggia/actions)
[![Unlicense](https://img.shields.io/github/license/MrKloan/loggia)](./UNLICENSE)

> A *loggia* is a piece of architecture that opens up a building to the outside world.

Loggia is a unified solution for deploying a document sharing platform with access level control. Its simple and
straight-forward features are particularly well-suited for a small non-profit organization context.

## Project setup

```shell script
# Run tests
$ ./mvnw clean test 

# Package and run
$ ./mvnw clean install
$ java -jar loggia-api/target/loggia-api-${version}-.jar
```