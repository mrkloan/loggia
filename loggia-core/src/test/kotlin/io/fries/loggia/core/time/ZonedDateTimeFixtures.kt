package io.fries.loggia.core.time

import java.time.ZoneId
import java.time.ZonedDateTime

fun unixTime(): ZonedDateTime = ZonedDateTime.of(1970, 1, 1, 0, 0, 0, 0, ZoneId.of("UTC"))
