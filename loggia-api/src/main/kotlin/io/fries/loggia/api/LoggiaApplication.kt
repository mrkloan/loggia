package io.fries.loggia.api

import org.springframework.beans.factory.annotation.Value
import org.springframework.boot.autoconfigure.SpringBootApplication
import org.springframework.boot.runApplication
import org.springframework.context.annotation.Bean
import org.springframework.context.annotation.Configuration
import java.time.ZoneId
import java.time.ZonedDateTime

fun main(args: Array<String>) {
    runApplication<LoggiaApplication>(*args)
}

@SpringBootApplication
class LoggiaApplication

@Configuration
class LoggiaConfiguration {

    @Bean
    fun clock(@Value("\${loggia.clock.timezone}") timezone: String): () -> ZonedDateTime = {
        ZonedDateTime.now(ZoneId.of(timezone))
    }
}