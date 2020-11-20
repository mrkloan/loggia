package io.fries.loggia.api.security.audit

import org.springframework.stereotype.Component
import org.springframework.web.filter.OncePerRequestFilter
import javax.servlet.FilterChain
import javax.servlet.http.HttpServletRequest
import javax.servlet.http.HttpServletResponse

@Component
class AuditFilter(
        private val mdc: MdcWrapper,
        private val supplyCorrelationId: () -> String
) : OncePerRequestFilter() {

    companion object {
        private const val CORRELATION_HEADER = "X-Correlation-ID"
        private const val CORRELATION_KEY = "correlationId"
        private val CORRELATION_REGEX = Regex("[a-zA-Z0-9-_/]+")
    }

    override fun doFilterInternal(request: HttpServletRequest, response: HttpServletResponse, chain: FilterChain) {
        try {
            mdc.put(CORRELATION_KEY, correlationIdFrom(request))
            chain.doFilter(request, response)
        } finally {
            mdc.remove(CORRELATION_KEY)
        }
    }

    private fun correlationIdFrom(request: HttpServletRequest): String {
        val correlationId = request.getHeader(CORRELATION_HEADER)

        return when {
            isValid(correlationId) -> correlationId
            else -> supplyCorrelationId()
        }
    }

    private fun isValid(correlationId: String?) = correlationId?.matches(CORRELATION_REGEX) ?: false
}