Global / onChangedBuildSource := ReloadOnSourceChanges

// give the user a nice default project!
ThisBuild / organization := "com.example"
ThisBuild / scalaVersion := "3.3.6"

lazy val root = (project in file(".")).settings(
  name := "middle-server",
  libraryDependencies ++= Seq(
    "org.flinkextended" %% "flink-scala-api-1" % "1.2.7",
    "org.apache.flink" % "flink-clients" % "1.20.1" % Provided,
    "org.apache.logging.log4j" % "log4j-slf4j2-impl" % "2.24.3"
  )
)

// make run command include the provided dependencies
Compile / run := Defaults
  .runTask(
    Compile / fullClasspath,
    Compile / run / mainClass,
    Compile / run / runner
  )
  .evaluated

// stays inside the sbt console when we press "ctrl-c" while a Flink programme executes with "run" or "runMain"
Compile / run / fork := true
Global / cancelable := true
