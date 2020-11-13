package io.fries.loggia.api.security.audit

import org.springframework.web.filter.OncePerRequestFilter
import javax.servlet.FilterChain
import javax.servlet.http.HttpServletRequest
import javax.servlet.http.HttpServletResponse

class AuditFilter(
        private val mdc: MdcWrapper,
        private val supplyCorrelationId: () -> String
) : OncePerRequestFilter() {

    companion object {
        private const val CORRELATION_HEADER = "X-Correlation-Id"
        private const val CORRELATION_KEY = "correlationId"
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
            correlationId.isNullOrBlank() -> supplyCorrelationId()
            else -> correlationId
        }
    }
}