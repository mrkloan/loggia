package io.fries.loggia.api.security.audit

import com.nhaarman.mockitokotlin2.given
import com.nhaarman.mockitokotlin2.mock
import com.nhaarman.mockitokotlin2.verify
import net.jqwik.api.*
import net.jqwik.api.lifecycle.BeforeTry
import org.assertj.core.api.Assertions.assertThatExceptionOfType
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.ValueSource
import java.util.function.Predicate.not
import javax.servlet.FilterChain
import javax.servlet.http.HttpServletRequest
import javax.servlet.http.HttpServletResponse

internal class AuditFilterTest {

    companion object {
        private const val SUPPLIED_CORRELATION_ID = "a-supplied-correlation-id"
    }

    private lateinit var mdcWrapper: MdcWrapper
    private lateinit var auditFilter: AuditFilter

    private lateinit var theRequest: HttpServletRequest
    private lateinit var theResponse: HttpServletResponse
    private lateinit var theChain: FilterChain

    @BeforeEach
    @BeforeTry
    internal fun setUp() {
        mdcWrapper = mock()
        auditFilter = AuditFilter(mdcWrapper) { SUPPLIED_CORRELATION_ID }

        theRequest = mock()
        theResponse = mock()
        theChain = mock()
    }

    @Property
    internal fun `Should set the Correlation-ID given the correlation header only has alphanumeric and allowed special chars`(@ForAll("correlationIds") correlationId: String) {
        `given a request with a correlation header of`(correlationId)

        auditFilter.doFilter(theRequest, theResponse, theChain)

        `then the correlation is`(correlationId)
        `then the next filter is triggered`()
    }

    @Provide
    fun correlationIds(): Arbitrary<String> = Arbitraries.strings()
            .alpha()
            .numeric()
            .withChars('-', '_', '/')
            .filter(not(String::isBlank))
            .unique()

    @Test
    internal fun `Should supply a Correlation-ID given the correlation header is not set`() {
        `given a request without correlation header`()

        `when the filter is triggered`()

        `then the correlation is`(SUPPLIED_CORRELATION_ID)
        `then the next filter is triggered`()
    }

    @ParameterizedTest
    @ValueSource(strings = ["", " ", "\n", "\r", "\t"])
    internal fun `Should supply a Correlation-ID given the correlation header is blank`(blankHeader: String) {
        `given a request with a correlation header of`(blankHeader)

        `when the filter is triggered`()

        `then the correlation is`(SUPPLIED_CORRELATION_ID)
        `then the next filter is triggered`()
    }

    @Property
    internal fun `Should supply a Correlation-ID given the correlation header is malformed`(@ForAll("malformedCorrelationIds") malformedCorrelationId: String) {
        `given a request with a correlation header of`(malformedCorrelationId)

        `when the filter is triggered`()

        `then the correlation is`(SUPPLIED_CORRELATION_ID)
        `then the next filter is triggered`()
    }

    @Provide
    fun malformedCorrelationIds(): Arbitrary<String> = Arbitraries.strings()
            .all()
            .filter(not(String::isBlank))
            .unique()

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

    private fun `then the correlation is`(expectedCorrelationId: String) = verify(mdcWrapper).put("correlationId", expectedCorrelationId)
    private fun `then the correlation id is removed`() = verify(mdcWrapper).remove("correlationId")
    private fun `then the next filter is triggered`() = verify(theChain).doFilter(theRequest, theResponse)
}
