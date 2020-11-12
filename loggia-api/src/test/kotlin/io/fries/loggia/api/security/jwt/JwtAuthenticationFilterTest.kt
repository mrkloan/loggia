package io.fries.loggia.api.security.jwt

import com.nhaarman.mockito_kotlin.*
import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test
import org.mockito.ArgumentCaptor
import org.springframework.security.authentication.UsernamePasswordAuthenticationToken
import org.springframework.security.core.GrantedAuthority
import org.springframework.security.core.context.SecurityContext
import org.springframework.security.core.userdetails.User
import org.springframework.security.web.authentication.WebAuthenticationDetailsSource
import javax.servlet.FilterChain
import javax.servlet.http.HttpServletRequest
import javax.servlet.http.HttpServletResponse

internal class JwtAuthenticationFilterTest {

    private val jwtUserDetailsService: JwtUserDetailsService = mock()
    private val jwtService: JwtService = mock()
    private val securityContext: SecurityContext = mock()

    private val theRequest: HttpServletRequest = mock()
    private val theResponse: HttpServletResponse = mock()
    private val theChain: FilterChain = mock()

    private lateinit var jwtAuthenticationFilter: JwtAuthenticationFilter

    @BeforeEach
    internal fun setUp() {
        this.jwtAuthenticationFilter = JwtAuthenticationFilter(jwtUserDetailsService, jwtService, securityContext)
    }

    @Test
    internal fun `Should validate the authentication given a valid token`() {
        val aUsername = "a-username"
        val someAuthorities = `given some authorities`()
        `given a request with a valid authentication token for user`(aUsername, someAuthorities)

        `when the filter is triggered`()

        `then the user is authenticated with`(aUsername, someAuthorities)
        `then the next filter is triggered`()
    }

    @Test
    internal fun `Should not authenticate any user given a request without an Authorization header`() {
        `given a request without authorization header`()

        `when the filter is triggered`()

        `then there is no authentication`()
        `then the next filter is triggered`()
    }

    @Test
    internal fun `Should not authenticate any user given an empty Authorization header`() {
        `given a request with an empty authorization header`()

        `when the filter is triggered`()

        `then there is no authentication`()
        `then the next filter is triggered`()
    }

    @Test
    internal fun `Should not authenticate any user given a non-bearer Authorization header`() {
        `given a request with a non-bearer authorization header`()

        `when the filter is triggered`()

        `then there is no authentication`()
        `then the next filter is triggered`()
    }

    private fun `given some authorities`(): List<GrantedAuthority> {
        return listOf(
                authorityOf("ROLE1"),
                authorityOf("ROLE2"),
                authorityOf("ROLE3"),
        )
    }

    private fun authorityOf(authority: String): GrantedAuthority = GrantedAuthority { authority }

    private fun `given a request with a valid authentication token for user`(username: String, authorities: List<GrantedAuthority>) {
        val token = "valid-jwt-token"
        val userDetails = User(username, "password", authorities)

        given(theRequest.getHeader("Authorization")).willReturn("Bearer $token")
        given(jwtService.subjectOf(token)).willReturn(username)
        given(jwtUserDetailsService.loadUserByUsername(username)).willReturn(userDetails)
        given(jwtService.isValid(token, userDetails)).willReturn(true)
    }

    private fun `given a request without authorization header`() {
        given(theRequest.getHeader("Authorization")).willReturn(null)
    }

    private fun `given a request with an empty authorization header`() {
        given(theRequest.getHeader("Authorization")).willReturn("")
    }

    private fun `given a request with a non-bearer authorization header`() {
        given(theRequest.getHeader("Authorization")).willReturn("some-token")
    }

    private fun `when the filter is triggered`() {
        jwtAuthenticationFilter.doFilter(theRequest, theResponse, theChain)
    }

    private fun `then the user is authenticated with`(expectedUsername: String, expectedAuthorities: List<GrantedAuthority>) {
        val authenticationCaptor = ArgumentCaptor.forClass(UsernamePasswordAuthenticationToken::class.java)
        verify(securityContext).authentication = authenticationCaptor.capture()

        val theAuthentication = authenticationCaptor.value
        assertThat(theAuthentication.principal).isEqualTo(expectedUsername)
        assertThat(theAuthentication.credentials).isNull()
        assertThat(theAuthentication.authorities).containsExactlyElementsOf(expectedAuthorities)
        assertThat(theAuthentication.details).isEqualTo(WebAuthenticationDetailsSource().buildDetails(theRequest))
    }

    private fun `then there is no authentication`() {
        verify(securityContext, never()).authentication = any()
    }

    private fun `then the next filter is triggered`() {
        verify(theChain).doFilter(theRequest, theResponse)
    }
}
