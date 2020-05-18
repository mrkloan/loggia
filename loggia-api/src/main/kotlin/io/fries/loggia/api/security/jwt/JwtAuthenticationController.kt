package io.fries.loggia.api.security.jwt

import org.springframework.http.ResponseEntity
import org.springframework.security.authentication.AuthenticationManager
import org.springframework.security.authentication.UsernamePasswordAuthenticationToken
import org.springframework.web.bind.annotation.PostMapping
import org.springframework.web.bind.annotation.RequestBody
import org.springframework.web.bind.annotation.RestController

data class JwtResponse(val token: String)
data class JwtRequest(val username: String, val password: String) {
    fun asAuthentication() = UsernamePasswordAuthenticationToken(username, password)
}

@RestController
class JwtAuthenticationController(
        private val authenticationManager: AuthenticationManager,
        private val jwtService: JwtService,
        private val jwtUserDetailsService: JwtUserDetailsService
) {

    @PostMapping("/login")
    fun login(@RequestBody jwtRequest: JwtRequest): ResponseEntity<JwtResponse> {
        authenticationManager.authenticate(jwtRequest.asAuthentication())

        val userDetails = jwtUserDetailsService.loadUserByUsername(jwtRequest.username)
        val token = jwtService.generateToken(userDetails)

        return ResponseEntity.ok(JwtResponse(token))
    }
}