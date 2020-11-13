package io.fries.loggia.api.security.jwt

import com.nhaarman.mockitokotlin2.mock
import com.nhaarman.mockitokotlin2.verify
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test
import org.springframework.security.core.AuthenticationException
import javax.servlet.http.HttpServletRequest
import javax.servlet.http.HttpServletResponse
import javax.servlet.http.HttpServletResponse.SC_UNAUTHORIZED

internal class JwtAuthenticationEntryPointTest {

    private lateinit var jwtAuthenticationEntryPoint: JwtAuthenticationEntryPoint

    @BeforeEach
    internal fun setUp() {
        this.jwtAuthenticationEntryPoint = JwtAuthenticationEntryPoint()
    }

    @Test
    internal fun `Should send an unauthorized response given any request, response or exception`() {
        val anyRequest: HttpServletRequest = mock()
        val anyResponse: HttpServletResponse = mock()
        val anyException: AuthenticationException = mock()

        jwtAuthenticationEntryPoint.commence(anyRequest, anyResponse, anyException)

        verify(anyResponse).sendError(SC_UNAUTHORIZED, "Unauthorized")
    }
}