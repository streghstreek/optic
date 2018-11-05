package com.opticdev.sdk.skills_sdk.lens

import com.opticdev.common.SchemaRef
import com.opticdev.parsers.rules.Rule
import com.opticdev.sdk.descriptions.Location
import com.opticdev.sdk.descriptions.enums.LocationEnums
import com.opticdev.sdk.skills_sdk.OMRange

case class OMLensSchemaComponent(schemaRef: SchemaRef,
                                 unique: Boolean = false,
                                 toMap: Option[String] = None, //key property. when defined mapping will be applied to an object keyed by this string
                                 inContainer: Option[String] = None //only will search a container if defined, global if empty
                                ) extends OMLensComponent {
  override def rules: Vector[Rule] = Vector()
  def `type`: OMLensComponentType = NotSupported

  def locationForCompiler: Option[Location] = {
    if (inContainer.isDefined) {
      Some(Location(LocationEnums.InContainer(inContainer.get)))
    } else {
      Some(Location(LocationEnums.InCurrentLens))
    }
  }

  def yieldsArray : Boolean = toMap.isEmpty
  def yieldsObject : Boolean = toMap.isDefined

}

case class OMComponentWithPropertyPath[T <: OMLensComponent](propertyPath: Seq[String], component: T) {
  def range: OMRange = component match {
    case f: OMLensNodeFinder => f.range
    case _ => OMRange(0,0)
  }
}