package io.fries.loggia.api.security.jwt

import org.springframework.security.core.userdetails.User
import org.springframework.security.core.userdetails.UserDetailsService
import org.springframework.security.core.userdetails.UsernameNotFoundException
import org.springframework.stereotype.Service

@Service
class JwtUserDetailsService : UserDetailsService {

    /**
     * TODO: Remove hardcoded authentication (user/password)
     */
    override fun loadUserByUsername(username: String?) = if ("user" == username) {
        User("user", "$2a$10\$slYQmyNdGzTn7ZLBXBChFOC9f6kFjAqPhccnP6DxlWXx2lPk1C3G6", ArrayList())
    } else {
        throw UsernameNotFoundException("User [$username] could not be found.")
    }
}