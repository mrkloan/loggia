package io.fries.loggia.api.security.jwt

import org.springframework.security.core.AuthenticationException
import org.springframework.security.web.AuthenticationEntryPoint
import org.springframework.stereotype.Component
import javax.servlet.http.HttpServletRequest
import javax.servlet.http.HttpServletResponse
import javax.servlet.http.HttpServletResponse.SC_UNAUTHORIZED

@Component
class JwtAuthenticationEntryPoint : AuthenticationEntryPoint {

    override fun commence(request: HttpServletRequest, response: HttpServletResponse, authenticationException: AuthenticationException) {
        response.sendError(SC_UNAUTHORIZED, "Unauthorized")
    }
}