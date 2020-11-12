package io.fries.loggia.api.security.jwt

import io.jsonwebtoken.JwtException
import org.springframework.security.authentication.UsernamePasswordAuthenticationToken
import org.springframework.security.core.context.SecurityContext
import org.springframework.security.web.authentication.WebAuthenticationDetailsSource
import org.springframework.stereotype.Component
import org.springframework.web.filter.OncePerRequestFilter
import javax.servlet.FilterChain
import javax.servlet.http.HttpServletRequest
import javax.servlet.http.HttpServletResponse

@Component
class JwtAuthenticationFilter(
        private val jwtUserDetailsService: JwtUserDetailsService,
        private val jwtService: JwtService,
        private val securityContext: SecurityContext
) : OncePerRequestFilter() {

    companion object {
        private const val AUTHORIZATION_HEADER = "Authorization"
        private const val BEARER_PREFIX = "Bearer "
    }

    override fun doFilterInternal(request: HttpServletRequest, response: HttpServletResponse, chain: FilterChain) {
        performAuthentication(request)
        chain.doFilter(request, response)
    }

    private fun performAuthentication(request: HttpServletRequest) {
        try {
            val authorization = request.getHeader(AUTHORIZATION_HEADER)

            if (authorization != null && authorization.startsWith(BEARER_PREFIX)) {
                validateAuthentication(authorization.substring(BEARER_PREFIX.length), request)
            }
        } catch (e: JwtException) {
            // TODO: use a proper logger
            e.printStackTrace()
        }
    }

    private fun validateAuthentication(token: String, request: HttpServletRequest) {
        val username = jwtService.subjectOf(token)
        val userDetails = jwtUserDetailsService.loadUserByUsername(username)

        if (jwtService.isValid(token, userDetails)) {
            val authentication = UsernamePasswordAuthenticationToken(userDetails.username, null, userDetails.authorities)
            authentication.details = WebAuthenticationDetailsSource().buildDetails(request)

            securityContext.authentication = authentication
        }
    }
}