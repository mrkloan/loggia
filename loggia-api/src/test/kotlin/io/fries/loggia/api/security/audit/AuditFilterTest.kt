package io.fries.loggia.api.security.audit

import com.nhaarman.mockito_kotlin.given
import com.nhaarman.mockito_kotlin.mock
import com.nhaarman.mockito_kotlin.verify
import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.ValueSource
import org.slf4j.MDC
import javax.servlet.FilterChain
import javax.servlet.http.HttpServletRequest
import javax.servlet.http.HttpServletResponse

internal class AuditFilterTest {

    companion object {
        private const val SUPPLIED_CORRELATION_ID = "a-generated-correlation-id"
    }

    private val theRequest: HttpServletRequest = mock()
    private val theResponse: HttpServletResponse = mock()
    private val theChain: FilterChain = mock()

    private lateinit var auditFilter: AuditFilter

    @BeforeEach
    internal fun setUp() {
        this.auditFilter = AuditFilter { SUPPLIED_CORRELATION_ID }
    }

    @Test
    internal fun `Should set the Correlation-Id given the request header is set`() {
        given(theRequest.getHeader("X-Correlation-Id")).willReturn("aCorrelationId")

        auditFilter.doFilter(theRequest, theResponse, theChain)

        assertThat(MDC.get("correlationId")).isEqualTo("aCorrelationId")
        verify(theChain).doFilter(theRequest, theResponse)
    }

    @Test
    internal fun `Should supply a Correlation-Id given the request header is not set`() {
        given(theRequest.getHeader("X-Correlation-Id")).willReturn(null)

        auditFilter.doFilter(theRequest, theResponse, theChain)

        assertThat(MDC.get("correlationId")).isEqualTo(SUPPLIED_CORRELATION_ID)
        verify(theChain).doFilter(theRequest, theResponse)
    }

    @ParameterizedTest
    @ValueSource(strings = ["", " ", "\n", "\r", "\t"])
    internal fun `Should supply a Correlation-Id given the request header is blank`(blankHeader: String) {
        given(theRequest.getHeader("X-Correlation-Id")).willReturn(blankHeader)

        auditFilter.doFilter(theRequest, theResponse, theChain)

        assertThat(MDC.get("correlationId")).isEqualTo(SUPPLIED_CORRELATION_ID)
        verify(theChain).doFilter(theRequest, theResponse)
    }
}