package io.fries.loggia.api.security.audit

import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test
import org.slf4j.MDC

internal class MdcWrapperTest {

    private lateinit var mdcWrapper: MdcWrapper

    @BeforeEach
    internal fun setUp() {
        this.mdcWrapper = MdcWrapper()
    }

    @Test
    internal fun `Should put value in MDC`() {
        mdcWrapper.put("key", "value")

        assertThat(MDC.get("key")).isEqualTo("value")
    }

    @Test
    internal fun `Should remove value from MDC`() {
        MDC.put("key", "value")

        mdcWrapper.remove("key")

        assertThat(MDC.get("key")).isNull()
    }
}