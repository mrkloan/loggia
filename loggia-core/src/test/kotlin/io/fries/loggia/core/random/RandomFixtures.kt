package io.fries.loggia.core.random

import kotlin.random.Random

class RandomFixtures(seed: Long) {

    private val random = Random(seed)

    fun anyStringStartingWith(prefix: String) = "$prefix-${random.nextInt()}"
}