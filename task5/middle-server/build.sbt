Global / onChangedBuildSource := ReloadOnSourceChanges

// give the user a nice default project!
ThisBuild / organization := "com.example"
ThisBuild / scalaVersion := "3.3.6"

lazy val flinkJob = (project in file("flink-job"))
  .settings(
    name := "flink-job",
    libraryDependencies ++= Seq(
      "org.flinkextended" %% "flink-scala-api-1" % "1.2.7",
      "org.apache.flink" % "flink-clients" % "1.20.1" % Provided,
      "org.apache.logging.log4j" % "log4j-slf4j2-impl" % "2.24.3",
      "org.playframework" %% "play" % "3.0.7",
      "org.scala-lang" %% "toolkit" % "0.7.0"
    )
  )

val http4sVersion = "0.23.23"

lazy val wsServer = (project in file("ws-server")).settings(
  name := "ws-server",
  libraryDependencies ++= Seq(
    "org.http4s" %% "http4s-ember-server" % http4sVersion,
    "org.http4s" %% "http4s-ember-client" % http4sVersion,
    "org.http4s" %% "http4s-dsl" % http4sVersion,
    "org.http4s" %% "http4s-server" % http4sVersion,
    "org.http4s" %% "http4s-client" % http4sVersion,
    "co.fs2" %% "fs2-core" % "3.9.2",
    "ch.qos.logback" % "logback-classic" % "1.4.11"
  )
)

lazy val root = (project in file("."))
  .aggregate(flinkJob, wsServer)

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
