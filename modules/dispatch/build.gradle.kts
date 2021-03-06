group = "com.xenoterracide.brix"
version = "0.1.0"

plugins {
  id("brix.bom")
  id("brix.java-convention")
}

dependencies {
  implementation(projects.configLoader.service)
  implementation(projects.processor.api)
  implementation(projects.cli.api)
  implementation(libs.commons.lang)
  implementation(libs.spring.context)
  implementation(libs.vavr)
}
