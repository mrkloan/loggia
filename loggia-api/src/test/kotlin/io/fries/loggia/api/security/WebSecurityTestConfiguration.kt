package io.fries.loggia.api.security

import io.fries.loggia.api.security.jwt.JwtAuthenticationEntryPoint
import io.fries.loggia.api.security.jwt.JwtService
import io.fries.loggia.api.security.jwt.JwtUserDetailsService
import org.springframework.boot.test.context.TestConfiguration
import org.springframework.boot.test.mock.mockito.MockBean
import org.springframework.security.authentication.AuthenticationManager
import org.springframework.security.core.context.SecurityContext

@TestConfiguration
internal class WebSecurityTestConfiguration {

    @MockBean
    private lateinit var securityContext: SecurityContext

    @MockBean
    private lateinit var jwtAuthenticationEntryPoint: JwtAuthenticationEntryPoint

    @MockBean
    private lateinit var jwtUserDetailsService: JwtUserDetailsService

    @MockBean
    private lateinit var authenticationManager: AuthenticationManager

    @MockBean
    private lateinit var jwtService: JwtService
}