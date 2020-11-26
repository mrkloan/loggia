package io.fries.loggia.core.member

data class Name(
        private val firstName: String,
        private val lastName: String
) {
    companion object {
        private val NAME_REGEX = Regex("^[a-z-A-ZÀ-Ÿ.'-, ]+$")
    }

    init {
        if(!firstName.matches(NAME_REGEX)) {
            throw IllegalArgumentException("Malformed firstName: [$firstName].")
        }
        if(!lastName.matches(NAME_REGEX)) {
            throw IllegalArgumentException("Malformed lastName: [$lastName].")
        }
    }
}