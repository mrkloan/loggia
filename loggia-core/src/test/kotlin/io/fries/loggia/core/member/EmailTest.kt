package io.fries.loggia.core.member

import net.jqwik.api.*
import net.jqwik.api.Combinators.combine
import org.assertj.core.api.Assertions.assertThat
import org.assertj.core.api.Assertions.assertThatExceptionOfType
import java.util.function.Predicate.not

internal class EmailTest {

    @Property
    internal fun `Should create an Email given a valid e-mail address`(@ForAll("emails") email: Email) {
        assertThat(email).isNotNull
    }

    @Property
    internal fun `Should normalize to lower case given a valid e-mail address`(@ForAll("validEmails") rawEmail: String) {
        val email = Email(rawEmail)
        val expectedEmail = Email(rawEmail.toLowerCase())

        assertThat(email).isEqualTo(expectedEmail)
    }

    @Property
    internal fun `Should throw given a malformed e-mail address`(@ForAll("malformedEmails") email: String) {
        assertThatExceptionOfType(IllegalArgumentException::class.java)
                .isThrownBy { Email(email) }
                .withMessage("Malformed e-mail address: [$email]")
    }

    @Provide
    fun validEmails(): Arbitrary<String> {
        val namePart = Arbitraries.strings()
                .alpha()
                .numeric()
                .withChars('!', '#', '$', '%', '&', '\'', '*', '+', '/', '=', '?', '^', '_', '`', '{', '|', '}', '~', '-')
                .ofMinLength(1)

        val domainPart = Arbitraries.strings()
                .alpha()
                .numeric()
                .withChars('-', '.')
                .ofMinLength(1)
                .filter(not { it.startsWith('.') })
                .filter(not { it.endsWith('.') })
                .filter(not { it.contains("..") })
                .filter(not { it.startsWith('-') })
                .filter(not { it.endsWith('-') })
                .filter(not { it.contains("--") })
                .filter(not { it.contains(".-") })
                .filter(not { it.contains("-.") })

        val topLevelDomainPart = Arbitraries.strings()
                .alpha()
                .withChars('.')
                .ofMinLength(3)
                .filter(not { it.startsWith('.') })
                .filter(not { it.endsWith('.') })
                .filter(not { it.contains("..") })

        return combine(namePart, domainPart, topLevelDomainPart).`as` { name, domain, topLevelDomain -> "$name@$domain.$topLevelDomain" }
    }

    @Provide
    fun malformedEmails(): Arbitrary<String> = Arbitraries.strings()
            .all()
            .unique()

    @Provide
    fun emails(): Arbitrary<Email> = validEmails().map(::Email)
}