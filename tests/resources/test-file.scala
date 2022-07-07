import scala.util.Random

val x: Int = Random.nextInt(10)

// @todo(2022-09-19): Add a default case
x match
  case 0 => "zero" 
  case 1 => "one"
  case 2 => "two" // @todo(2022-09-19): A TODO at the end of a line 
  case 3 => "three" 
