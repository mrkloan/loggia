package io.fries.loggia.core.member

import net.jqwik.api.*
import org.assertj.core.api.Assertions.assertThat
import org.assertj.core.api.Assertions.assertThatExceptionOfType
import java.util.function.Predicate.not

internal class NameTest {

    @Property
    internal fun `Should create a Name given valid first and last names`(@ForAll("names") name: Name) {
        assertThat(name).isNotNull
    }

    @Property
    internal fun `Should throw given a malformed first name`(@ForAll("malformedNames") firstName: String, @ForAll("validNames") lastName: String) {
        assertThatExceptionOfType(IllegalArgumentException::class.java)
                .isThrownBy { Name(firstName, lastName) }
                .withMessage("Malformed firstName: [$firstName].")
    }

    @Property
    internal fun `Should throw given a malformed last name`(@ForAll("validNames") firstName: String, @ForAll("malformedNames") lastName: String) {
        assertThatExceptionOfType(IllegalArgumentException::class.java)
                .isThrownBy { Name(firstName, lastName) }
                .withMessage("Malformed lastName: [$lastName].")
    }

    @Provide
    fun validNames(): Arbitrary<String> = Arbitraries.strings()
            .alpha()
            .withChars('À', 'Â', 'Ç', 'É', 'È', 'Ê', 'Ë', 'Î', 'Ï', 'Ô', 'Û', 'Ù', 'Ü', 'Ÿ', 'Ñ', 'Æ', 'Œ')
            .withChars('à', 'â', 'ç', 'é', 'è', 'ê', 'ë', 'î', 'ï', 'ô', 'û', 'ù', 'ü', 'ÿ', 'ñ', 'æ', 'œ')
            .withChars('.', '-', '\'', ',', ' ')
            .filter(not(String::isBlank))
            .unique()

    @Provide
    fun malformedNames(): Arbitrary<String> = Arbitraries.strings()
            .all()
            .unique()

    @Provide
    fun names(): Arbitrary<Name> = Combinators.combine(validNames(), validNames()).`as`(::Name)
}