package io.fries.loggia.api.security.jwt

import io.jsonwebtoken.Claims
import io.jsonwebtoken.Jwts
import io.jsonwebtoken.SignatureAlgorithm
import io.jsonwebtoken.SignatureAlgorithm.HS512
import org.springframework.beans.factory.annotation.Value
import org.springframework.security.core.userdetails.UserDetails
import org.springframework.stereotype.Service
import java.time.Duration
import java.util.*
import kotlin.collections.HashMap

@Service
class JwtService(
        @Value("\${loggia.jwt.secret}") private val secret: String,
        @Value("#{T(java.time.Duration).ofSeconds('\${loggia.jwt.validity}')}") private val validity: Duration
) {

    fun subjectOf(token: String): String = claimOf(token, Claims::getSubject)

    private fun expirationOf(token: String): Date = claimOf(token, Claims::getExpiration)

    fun isValid(token: String, userDetails: UserDetails) = subjectOf(token) == userDetails.username && !isTokenExpired(token)

    private fun isTokenExpired(token: String) = expirationOf(token).before(Date())

    private fun <T> claimOf(token: String, resolveClaim: (Claims) -> T) = resolveClaim(claimsOf(token))

    private fun claimsOf(token: String) = Jwts.parser()
            .setSigningKey(secret)
            .parseClaimsJws(token)
            .body

    fun generateToken(userDetails: UserDetails): String = generateToken(userDetails.username, mutableMapOf(
            "roles" to userDetails.authorities.map { it.authority }
    ))

    private fun generateToken(subject: String, claims: Map<String, Any>) = Jwts.builder()
            .setClaims(claims)
            .setSubject(subject)
            .setIssuedAt(Date())
            .setExpiration(Date(System.currentTimeMillis() + validity.toMillis()))
            .signWith(HS512, secret)
            .compact()
}