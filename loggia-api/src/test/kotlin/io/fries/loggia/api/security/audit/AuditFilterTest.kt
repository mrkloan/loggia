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
    internal fun `Should set the Correlation-ID given the request header is set`() {
        `given a request with a correlation header of`("aCorrelationId")

        `when the filter is triggered`()

        `then the correlation is`("aCorrelationId")
        `then the next filter is triggered`()
    }

    @Test
    internal fun `Should supply a Correlation-ID given the request header is not set`() {
        `given a request without correlation header`()

        `when the filter is triggered`()

        `then the correlation is`(SUPPLIED_CORRELATION_ID)
        `then the next filter is triggered`()
    }

    @ParameterizedTest
    @ValueSource(strings = ["", " ", "\n", "\r", "\t"])
    internal fun `Should supply a Correlation-ID given the request header is blank`(blankHeader: String) {
        `given a request with a correlation header of`(blankHeader)

        `when the filter is triggered`()

        `then the correlation is`(SUPPLIED_CORRELATION_ID)
        `then the next filter is triggered`()
    }

    @Test
    internal fun `Should remove the Correlation-ID from MDC given the filter chain has been executed`() {
        `given a request with a correlation header of`("aCorrelationId")

        `when the filter is triggered`()

        `then the next filter is triggered`()
        `then the correlation id is removed`()
    }

    @Test
    internal fun `Should remove the Correlation-ID from MDC given the filter chain has thrown`() {
        `given a request with a correlation header of`("aCorrelationId")
        given(theChain.doFilter(theRequest, theResponse)).willThrow(RuntimeException())

        assertThatExceptionOfType(RuntimeException::class.java)
                .isThrownBy { `when the filter is triggered`() }

        `then the correlation id is removed`()
    }

    private fun `given a request without correlation header`() = `given a request with a correlation header of`(null)
    private fun `given a request with a correlation header of`(correlationId: String?) = given(theRequest.getHeader("X-Correlation-ID")).willReturn(correlationId)

    private fun `when the filter is triggered`() = auditFilter.doFilter(theRequest, theResponse, theChain)

    private fun `then the next filter is triggered`() = verify(theChain).doFilter(theRequest, theResponse)
    private fun `then the correlation is`(expectedCorrelationId: String) = verify(mdcWrapper).put("correlationId", expectedCorrelationId)
    private fun `then the correlation id is removed`() = verify(mdcWrapper).remove("correlationId")
}
