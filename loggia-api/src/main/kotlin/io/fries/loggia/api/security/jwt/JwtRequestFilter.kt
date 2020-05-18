package io.fries.loggia.api.security.jwt

import org.springframework.security.authentication.UsernamePasswordAuthenticationToken
import org.springframework.security.core.context.SecurityContextHolder
import org.springframework.security.web.authentication.WebAuthenticationDetailsSource
import org.springframework.stereotype.Component
import org.springframework.web.filter.OncePerRequestFilter
import javax.servlet.FilterChain
import javax.servlet.http.HttpServletRequest
import javax.servlet.http.HttpServletResponse

@Component
class JwtRequestFilter(
        private val jwtUserDetailsService: JwtUserDetailsService,
        private val jwtService: JwtService
) : OncePerRequestFilter() {

    /**
     * TODO: Handle exceptions thrown in this filter.
     */
    override fun doFilterInternal(request: HttpServletRequest, response: HttpServletResponse, chain: FilterChain) {
        val authorization = request.getHeader("Authorization")

        if (authorization != null && authorization.startsWith("Bearer ")) {
            val token = authorization.substring(7)
            val username = jwtService.subjectOf(token)

            if (SecurityContextHolder.getContext().authentication == null) {
                val userDetails = jwtUserDetailsService.loadUserByUsername(username)

                if (jwtService.isValid(token, userDetails)) {
                    val authentication = UsernamePasswordAuthenticationToken(userDetails, null, userDetails.authorities)
                    authentication.details = WebAuthenticationDetailsSource().buildDetails(request)

                    SecurityContextHolder.getContext().authentication = authentication
                }
            }
        }

        chain.doFilter(request, response)
    }
}