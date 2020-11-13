package io.fries.loggia.api.security.audit

import com.nhaarman.mockito_kotlin.given
import com.nhaarman.mockito_kotlin.mock
import com.nhaarman.mockito_kotlin.verify
import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test
import org.slf4j.MDC
import javax.servlet.FilterChain
import javax.servlet.http.HttpServletRequest
import javax.servlet.http.HttpServletResponse

internal class AuditFilterTest {

    private val theRequest: HttpServletRequest = mock()
    private val theResponse: HttpServletResponse = mock()
    private val theChain: FilterChain = mock()

    private lateinit var auditFilter: AuditFilter

    @BeforeEach
    internal fun setUp() {
        this.auditFilter = AuditFilter()
    }

    @Test
    internal fun `Should set the Correlation-ID given the request header is set`() {
        given(theRequest.getHeader("X-Correlation-Id")).willReturn("aCorrelationId")

        auditFilter.doFilter(theRequest, theResponse, theChain)

        assertThat(MDC.get("correlationId")).isEqualTo("aCorrelationId")
        verify(theChain).doFilter(theRequest, theResponse)
    }
}