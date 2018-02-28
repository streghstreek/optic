package com.opticdev.arrow

import better.files.File
import com.opticdev.arrow.index.IndexSourceGear
import com.opticdev.common.PackageRef
import com.opticdev.core.sourcegear.project.config.ProjectFile
import com.opticdev.core.sourcegear.{GearSet, SGConstructor, SourceGear}
import com.opticdev.sdk.descriptions.{Schema, SchemaRef, Transformation}
import play.api.libs.json.JsObject

import scala.concurrent.duration._
import scala.concurrent.Await

object ExampleSourcegears {

  lazy val sgWithTransformations = new {
    val schemaModel = Schema(SchemaRef(PackageRef("optic:test"), "model"), JsObject.empty)
    val schemaRoute = Schema(SchemaRef(PackageRef("optic:test"), "route"), JsObject.empty)
    val schemaForm = Schema(SchemaRef(PackageRef("optic:test"), "form"), JsObject.empty)
    val schemaFetch = Schema(SchemaRef(PackageRef("optic:test"), "fetch"), JsObject.empty)

    val transformationPackage = PackageRef("optic:test-transform")

    val sourceGear = new SourceGear {
      override val parsers = Set()
      override val gearSet = new GearSet()
      override val schemas = Set(schemaModel, schemaRoute, schemaForm, schemaFetch)
      override val transformations = Set(
        Transformation("Model -> Route", transformationPackage,  schemaModel.schemaRef, schemaRoute.schemaRef, ""),
        Transformation("Route -> Form", transformationPackage, schemaRoute.schemaRef, schemaForm.schemaRef, ""),
        Transformation("Route -> Fetch", transformationPackage, schemaRoute.schemaRef, schemaFetch.schemaRef, "")
      )
    }

    val knowledgeGraph = IndexSourceGear.runFor(sourceGear)

  }

  lazy val exampleProjectSG = new {

    val sourceGear = {
      val future = SGConstructor.fromProjectFile(new ProjectFile(File("test-examples/resources/example_packages/express/optic.yaml")))
      Await.result(future, 10 seconds).inflate
    }

    val knowledgeGraph = IndexSourceGear.runFor(sourceGear)

  }
}