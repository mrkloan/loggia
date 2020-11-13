package io.fries.loggia.api.security.audit

import org.slf4j.MDC
import org.springframework.web.filter.OncePerRequestFilter
import javax.servlet.FilterChain
import javax.servlet.http.HttpServletRequest
import javax.servlet.http.HttpServletResponse

class AuditFilter(val supplyCorrelationId: () -> String) : OncePerRequestFilter() {

    override fun doFilterInternal(request: HttpServletRequest, response: HttpServletResponse, chain: FilterChain) {
        MDC.put("correlationId", request.getHeader("X-Correlation-Id") ?: supplyCorrelationId())
        chain.doFilter(request, response)
    }
}