package io.fries.loggia.api.security.jwt

import io.jsonwebtoken.Claims
import io.jsonwebtoken.Jwts
import io.jsonwebtoken.SignatureAlgorithm.HS512
import org.springframework.beans.factory.annotation.Value
import org.springframework.security.core.Authentication
import org.springframework.security.core.userdetails.UserDetails
import org.springframework.stereotype.Service
import java.time.Duration
import java.time.ZonedDateTime
import java.util.*

@Service
class JwtService(
        @Value("\${loggia.security.jwt.secret}") private val secret: String,
        @Value("#{T(java.time.Duration).ofSeconds('\${loggia.security.jwt.validity}')}") private val validity: Duration,
        private val clock: () -> ZonedDateTime
) {

    fun subjectOf(token: String): String = claimOf(token, Claims::getSubject)

    fun isValid(token: String, userDetails: UserDetails) = subjectOf(token) == userDetails.username && !isTokenExpired(token)

    private fun isTokenExpired(token: String) = expirationOf(token).before(Date.from(clock().toInstant()))

    private fun expirationOf(token: String): Date = claimOf(token, Claims::getExpiration)

    private fun <T> claimOf(token: String, resolveClaim: (Claims) -> T) = resolveClaim(claimsOf(token))

    private fun claimsOf(token: String) = Jwts.parser()
            .setClock { Date.from(clock().toInstant()) }
            .setSigningKey(secret)
            .parseClaimsJws(token)
            .body

    fun generateTokenFor(authentication: Authentication): String = generateToken(
            authentication.name,
            mutableMapOf("roles" to authentication.authorities.map { it.authority })
    )

    private fun generateToken(subject: String, claims: Map<String, Any>) = Jwts.builder()
            .setClaims(claims)
            .setSubject(subject)
            .setIssuedAt(Date.from(clock().toInstant()))
            .setExpiration(Date.from(clock().plus(validity).toInstant()))
            .signWith(HS512, secret)
            .compact()
}