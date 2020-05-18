package io.fries.loggia.api

import org.springframework.boot.autoconfigure.SpringBootApplication
import org.springframework.boot.runApplication

@SpringBootApplication
class LoggiaApplication

fun main(args: Array<String>) {
	runApplication<LoggiaApplication>(*args)
}
