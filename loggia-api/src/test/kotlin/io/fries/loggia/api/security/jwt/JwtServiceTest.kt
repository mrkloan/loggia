package io.fries.loggia.api.security.jwt

import io.fries.loggia.core.time.unixTime
import io.jsonwebtoken.MalformedJwtException
import io.jsonwebtoken.SignatureException
import org.assertj.core.api.Assertions.assertThat
import org.assertj.core.api.Assertions.assertThatExceptionOfType
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test
import org.springframework.security.authentication.UsernamePasswordAuthenticationToken
import org.springframework.security.core.GrantedAuthority
import org.springframework.security.core.userdetails.User
import java.time.Duration

internal class JwtServiceTest {

    private lateinit var jwtService: JwtService

    @BeforeEach
    internal fun setUp() {
        this.jwtService = JwtService("test-secret", Duration.ofSeconds(60)) { unixTime() }
    }

    @Test
    internal fun `Should extract the subject given a valid JWT`() {
        val aToken = "eyJhbGciOiJIUzUxMiJ9.eyJyb2xlcyI6W10sInN1YiI6ImxvZ2luIiwiaWF0IjowLCJleHAiOjYwfQ.JxhXooEa4O1Ld_e-ae2n1GubtY366m3O4CVmYJjp6BV4aGbwih5jtVXPrWVt_nJbqP70h7A_JO-G0sOYjSL9WA"

        val theSubject = jwtService.subjectOf(aToken)

        assertThat(theSubject).isEqualTo("login")
    }

    @Test
    internal fun `Should throw given a compromised JWT signature`() {
        val aToken = "eyJhbGciOiJIUzUxMiJ9.eyJyb2xlcyI6W10sInN1YiI6ImxvZ2luIiwiaWF0IjowLCJleHAiOjYwfQ.JxhXooEa4O1Ld_edae2n1GubtY366m3O4CVmYJjp6BV4aGbwih5jtVXPrWVt_nJbqP70h7A_JO-GOsOYjSL9WA"

        assertThatExceptionOfType(SignatureException::class.java)
                .isThrownBy { jwtService.subjectOf(aToken) }
    }

    @Test
    internal fun `Should throw given a malformed JWT payload`() {
        val aToken = "eyJgbGciOiJIUzUxMiJ0.eyJyb2xlcyI6W10sInN1YiI6ImxvZ2luIiwiaWF0IjowLCJleHAiOjYwfQ.JxhXooEa4O1Ld_e-ae2n1GubtY366m3O4CVmYJjp6BV4aGbwih5jtVXPrWVt_nJbqP70h7A_JO-G0sOYjSL9WA"

        assertThatExceptionOfType(MalformedJwtException::class.java)
                .isThrownBy { jwtService.subjectOf(aToken) }
    }

    @Test
    internal fun `Should validate JWT given a username matching the token subject`() {
        val aToken = "eyJhbGciOiJIUzUxMiJ9.eyJyb2xlcyI6W10sInN1YiI6ImxvZ2luIiwiaWF0IjowLCJleHAiOjYwfQ.JxhXooEa4O1Ld_e-ae2n1GubtY366m3O4CVmYJjp6BV4aGbwih5jtVXPrWVt_nJbqP70h7A_JO-G0sOYjSL9WA"
        val aUser = User("login", "", emptyList())

        val isValid = jwtService.isValid(aToken, aUser)

        assertThat(isValid).isTrue()
    }

    @Test
    internal fun `Should not validate JWT given a username not matching the token subject`() {
        val aToken = "eyJhbGciOiJIUzUxMiJ9.eyJyb2xlcyI6W10sInN1YiI6ImxvZ2luIiwiaWF0IjowLCJleHAiOjYwfQ.JxhXooEa4O1Ld_e-ae2n1GubtY366m3O4CVmYJjp6BV4aGbwih5jtVXPrWVt_nJbqP70h7A_JO-G0sOYjSL9WA"
        val aUser = User("INVALID_USERNAME", "", emptyList())

        val isValid = jwtService.isValid(aToken, aUser)

        assertThat(isValid).isFalse()
    }

    @Test
    internal fun `Should generate a JWT given an authentication with authorities`() {
        val anAuthentication = UsernamePasswordAuthenticationToken("login", "password", listOf(GrantedAuthority { "USER" }))

        val theToken = jwtService.generateTokenFor(anAuthentication)

        assertThat(theToken).isEqualTo("eyJhbGciOiJIUzUxMiJ9.eyJyb2xlcyI6WyJVU0VSIl0sInN1YiI6ImxvZ2luIiwiaWF0IjowLCJleHAiOjYwfQ.3faWwxQHfzaDjjfzUPI07Xzc7F1kcb7R9pGc-8QTyepWVadH8oxff5ATY_yEVns-8I1TwfMAQZfBxqsatBrrUQ")
    }
}
