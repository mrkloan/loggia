package io.fries.loggia.core.member

data class AccessLevel(private val level: Int) {

    init {
        if (level < 0) {
            throw IllegalArgumentException("Access level cannot be negative: [$level].")
        }
    }
}