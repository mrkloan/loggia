package io.fries.loggia.api

import io.fries.loggia.api.security.WebSecurityTestConfiguration
import org.springframework.boot.test.context.TestConfiguration
import org.springframework.boot.test.mock.mockito.MockBean
import org.springframework.context.annotation.Import
import java.time.ZonedDateTime

@TestConfiguration
@Import(WebSecurityTestConfiguration::class)
class LoggiaTestConfiguration {

    @MockBean
    private lateinit var supplyCorrelationId: () -> String

    @MockBean
    private lateinit var clock: () -> ZonedDateTime
}