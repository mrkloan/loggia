package io.fries.loggia.api.security.jwt

import org.springframework.http.ResponseEntity
import org.springframework.security.authentication.AuthenticationManager
import org.springframework.security.authentication.UsernamePasswordAuthenticationToken
import org.springframework.web.bind.annotation.PostMapping
import org.springframework.web.bind.annotation.RequestBody
import org.springframework.web.bind.annotation.RestController

data class JwtResponse(val token: String)
data class JwtRequest(val login: String, val password: String) {
    fun asAuthentication() = UsernamePasswordAuthenticationToken(login, password)
}

@RestController
class JwtAuthenticationController(
        private val authenticationManager: AuthenticationManager,
        private val jwtService: JwtService
) {

    @PostMapping("/login")
    fun login(@RequestBody jwtRequest: JwtRequest): ResponseEntity<JwtResponse> {
        val authentication = authenticationManager.authenticate(jwtRequest.asAuthentication())
        val token = jwtService.generateTokenFor(authentication)

        return ResponseEntity.ok(JwtResponse(token))
    }
}