package io.fries.loggia.api.security.audit

import com.nhaarman.mockitokotlin2.given
import com.nhaarman.mockitokotlin2.mock
import com.nhaarman.mockitokotlin2.verify
import org.assertj.core.api.Assertions.assertThatExceptionOfType
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.ValueSource
import javax.servlet.FilterChain
import javax.servlet.http.HttpServletRequest
import javax.servlet.http.HttpServletResponse

internal class AuditFilterTest {

    companion object {
        private const val SUPPLIED_CORRELATION_ID = "a-generated-correlation-id"
    }

    private val mdcWrapper: MdcWrapper = mock()

    private val theRequest: HttpServletRequest = mock()
    private val theResponse: HttpServletResponse = mock()
    private val theChain: FilterChain = mock()

    private lateinit var auditFilter: AuditFilter

    @BeforeEach
    internal fun setUp() {
        this.auditFilter = AuditFilter(mdcWrapper) { SUPPLIED_CORRELATION_ID }
    }

    @Test
    internal fun `Should set the Correlation-Id given the request header is set`() {
        given(theRequest.getHeader("X-Correlation-Id")).willReturn("aCorrelationId")

        auditFilter.doFilter(theRequest, theResponse, theChain)

        verify(mdcWrapper).put("correlationId", "aCorrelationId")
        verify(theChain).doFilter(theRequest, theResponse)
    }

    @Test
    internal fun `Should supply a Correlation-Id given the request header is not set`() {
        given(theRequest.getHeader("X-Correlation-Id")).willReturn(null)

        auditFilter.doFilter(theRequest, theResponse, theChain)

        verify(mdcWrapper).put("correlationId", SUPPLIED_CORRELATION_ID)
        verify(theChain).doFilter(theRequest, theResponse)
    }

    @ParameterizedTest
    @ValueSource(strings = ["", " ", "\n", "\r", "\t"])
    internal fun `Should supply a Correlation-Id given the request header is blank`(blankHeader: String) {
        given(theRequest.getHeader("X-Correlation-Id")).willReturn(blankHeader)

        auditFilter.doFilter(theRequest, theResponse, theChain)

        verify(mdcWrapper).put("correlationId", SUPPLIED_CORRELATION_ID)
        verify(theChain).doFilter(theRequest, theResponse)
    }

    @Test
    internal fun `Should remove the Correlation-Id from MDC given the filter chain has been executed`() {
        given(theRequest.getHeader("X-Correlation-Id")).willReturn("aCorrelationId")

        auditFilter.doFilter(theRequest, theResponse, theChain)

        verify(theChain).doFilter(theRequest, theResponse)
        verify(mdcWrapper).remove("correlationId")
    }

    @Test
    internal fun `Should remove the Correlation-Id from MDC given the filter chain has thrown`() {
        given(theRequest.getHeader("X-Correlation-Id")).willReturn("aCorrelationId")
        given(theChain.doFilter(theRequest, theResponse)).willThrow(RuntimeException())

        assertThatExceptionOfType(RuntimeException::class.java)
                .isThrownBy { auditFilter.doFilter(theRequest, theResponse, theChain) }

        verify(mdcWrapper).remove("correlationId")
    }
}