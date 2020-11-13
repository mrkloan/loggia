package io.fries.loggia.api.security.audit

import org.slf4j.MDC

class MdcWrapper {
    fun put(key: String, value: String): Unit = MDC.put(key, value)
    fun remove(key: String): Unit = MDC.remove(key)
}