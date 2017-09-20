package sourcegear.graph

import com.opticdev.parsers.AstGraph
import sourcegear.gears.parsing.ParseResult

import scalax.collection.edge.LkDiEdge
import scalax.collection.mutable.Graph
import scalax.collection.edge.LkDiEdge
import scalax.collection.mutable.Graph
import scalax.collection.edge.Implicits._

object GraphOperations {
  def addModelsToGraph(parseResults: Vector[ParseResult]) (implicit astGraph: AstGraph) : Unit = {
    parseResults.foreach(result=> {
      astGraph add (result.astNode ~+#> result.modelNode) (YieldsModel(result.parseGear))
    })
  }
}
