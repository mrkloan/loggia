package io.fries.loggia.core.random

import org.junit.jupiter.api.extension.ExtensionContext
import org.junit.jupiter.api.extension.ParameterContext
import org.junit.jupiter.api.extension.ParameterResolver
import kotlin.random.Random

class RandomExtension : ParameterResolver {

    override fun supportsParameter(parameterContext: ParameterContext, extensionContext: ExtensionContext): Boolean = parameterContext.parameter.type == RandomFixtures::class.java

    override fun resolveParameter(parameterContext: ParameterContext, extensionContext: ExtensionContext): Any {
        val seed = Random.Default.nextLong()
        println("# Random seed for ${extensionContext.uniqueId} is: $seed")

        return RandomFixtures(seed)
    }

}