package io.fries.loggia.core.member

import net.jqwik.api.*
import org.assertj.core.api.Assertions.assertThat
import org.assertj.core.api.Assertions.assertThatExceptionOfType
import java.util.function.Predicate.not

internal class AccessLevelTest {

    @Property
    internal fun `Should create AccessLevel given a positive access level`(@ForAll("accessLevels") accessLevel: AccessLevel) {
        assertThat(accessLevel).isNotNull
    }

    @Property
    internal fun `Should throw given a negative level`(@ForAll("negativeAccessLevels") accessLevel: Int) {
        assertThatExceptionOfType(IllegalArgumentException::class.java)
                .isThrownBy { AccessLevel(accessLevel) }
                .withMessage("Access level cannot be negative: [$accessLevel].")
    }

    @Provide
    fun positiveAccessLevels(): Arbitrary<Int> = Arbitraries.integers()
            .greaterOrEqual(0)
            .unique()

    @Provide
    fun negativeAccessLevels(): Arbitrary<Int> = Arbitraries.integers()
            .lessOrEqual(0)
            .filter(not(0::equals))
            .unique()

    @Provide
    fun accessLevels(): Arbitrary<AccessLevel> = positiveAccessLevels().map(::AccessLevel)
}