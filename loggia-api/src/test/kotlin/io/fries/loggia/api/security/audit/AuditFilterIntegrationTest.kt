package io.fries.loggia.api.security.audit

import com.nhaarman.mockitokotlin2.verify
import io.fries.loggia.api.LoggiaTestConfiguration
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.extension.ExtendWith
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.boot.test.autoconfigure.web.servlet.WebMvcTest
import org.springframework.context.annotation.Import
import org.springframework.test.context.junit.jupiter.SpringExtension
import org.springframework.test.web.servlet.MockMvc
import org.springframework.test.web.servlet.get

@ExtendWith(SpringExtension::class)
@Import(LoggiaTestConfiguration::class)
@WebMvcTest
class AuditFilterIntegrationTest {

    @Autowired
    private lateinit var mdcWrapper: MdcWrapper

    @Autowired
    private lateinit var mockMvc: MockMvc

    @Test
    internal fun `Should set the Correlation-ID given the request header is set`() {
        mockMvc.get("/") {
            header("X-Correlation-ID", "aCorrelationId")
        }

        verify(mdcWrapper).put("correlationId", "aCorrelationId")
    }
}