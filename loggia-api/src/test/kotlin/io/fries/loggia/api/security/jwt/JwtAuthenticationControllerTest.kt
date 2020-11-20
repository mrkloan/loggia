package io.fries.loggia.api.security.jwt

import com.nhaarman.mockitokotlin2.given
import com.nhaarman.mockitokotlin2.mock
import io.fries.loggia.api.LoggiaTestConfiguration
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.extension.ExtendWith
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.boot.test.autoconfigure.web.servlet.WebMvcTest
import org.springframework.context.annotation.Import
import org.springframework.http.MediaType.APPLICATION_JSON
import org.springframework.security.authentication.AuthenticationManager
import org.springframework.security.authentication.UsernamePasswordAuthenticationToken
import org.springframework.security.core.Authentication
import org.springframework.test.context.junit.jupiter.SpringExtension
import org.springframework.test.web.servlet.MockMvc
import org.springframework.test.web.servlet.post

@ExtendWith(SpringExtension::class)
@Import(LoggiaTestConfiguration::class)
@WebMvcTest(JwtAuthenticationController::class)
internal class JwtAuthenticationControllerTest {

    @Autowired
    private lateinit var authenticationManager: AuthenticationManager

    @Autowired
    private lateinit var jwtService: JwtService

    @Autowired
    private lateinit var mockMvc: MockMvc

    @Test
    internal fun `Should authenticate user and create a JWT token`() {
        val login = "the-login"
        val password = "the-password"
        val generatedToken = "jwt-token"

        `given a successful authentication generating a token`(login, password, generatedToken)

        mockMvc.post("/login") {
            accept = APPLICATION_JSON
            content = """{"login": "$login", "password": "$password"}"""
            contentType = APPLICATION_JSON
        }.andExpect {
            status { isOk }
            content { contentType(APPLICATION_JSON) }
            content { json("""{"token": "$generatedToken"}""") }
        }
    }

    @Suppress("SameParameterValue")
    private fun `given a successful authentication generating a token`(login: String, password: String, token: String) {
        val authenticationToken: Authentication = UsernamePasswordAuthenticationToken(login, password)
        val authenticatedUser: Authentication = mock()

        given(authenticationManager.authenticate(authenticationToken)).willReturn(authenticatedUser)
        given(jwtService.generateTokenFor(authenticatedUser)).willReturn(token)
    }
}