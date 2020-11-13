package io.fries.loggia.api.security.audit

import org.springframework.web.filter.OncePerRequestFilter
import javax.servlet.FilterChain
import javax.servlet.http.HttpServletRequest
import javax.servlet.http.HttpServletResponse

class AuditFilter(
        private val mdc: MdcWrapper,
        private val supplyCorrelationId: () -> String
) : OncePerRequestFilter() {

    override fun doFilterInternal(request: HttpServletRequest, response: HttpServletResponse, chain: FilterChain) {
        mdc.put("correlationId", correlationIdFrom(request))
        chain.doFilter(request, response)
    }

    private fun correlationIdFrom(request: HttpServletRequest): String {
        val correlationId = request.getHeader("X-Correlation-Id")

        return when {
            correlationId.isNullOrBlank() -> supplyCorrelationId()
            else -> correlationId
        }
    }
}