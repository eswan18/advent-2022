package day07

final case class FSLocation(path: Vector[String]):
    def cd(relPath: String): FSLocation =
        relPath match
            case "." => this
            case ".." => FSLocation(path.dropRight(1))
            case _ => FSLocation(path :+ relPath)

sealed trait Item {
    def name: String
    def size: Int
}

final case class Filesystem(root: Item):
    def at(loc: FSLocation): Item =
        // Remove the leading slash from the path
        val path = loc.path.drop(1)
        path.foldLeft(root) { (item, folderName) =>
            item match {
                case Folder(_, children) => children.find(_.name == folderName) match
                    case Some(folder: Folder) => folder
                    case Some(file: File) => throw new Exception(s"Invalid path -- $folderName is a file")
                    case None => throw new Exception(s"Invalid path -- $folderName not found")
                case _ => throw new Exception(s"Invalid path -- $item is not a folder")
            }
        }

    override def toString: String = root.toString

    def directories: Vector[Folder] =
        def loop(item: Item): Vector[Folder] =
            item match
                case folder: Folder => Vector(folder) ++ folder.children.flatMap(loop)
                case _ => Vector.empty
        loop(root)
    
    def size: Int = root.size
    
object Filesystem:
    def fromCommands(commands: Vector[Command]): Filesystem =
        val fs = Filesystem(Folder("/", Vector.empty))
        var cwd = FSLocation(Vector("/"))
        // Loop through the commands
        for command <- commands do
            command match
                case LS(listings) =>
                    // Add each listing to the current working directory
                    for listing <- listings do
                        val currentFolder = fs.at(cwd) match
                            case folder: Folder => folder
                            case _ => throw new Exception("Invalid path")
                        listing match
                            case folder: Folder => currentFolder.addItem(folder)
                            case file: File => currentFolder.addItem(file)
                    end for
                case CD("/") =>
                    cwd = FSLocation(Vector("/"))
                case CD(path) =>
                    cwd = cwd.cd(path)
        end for
        fs

final case class Folder(name: String, var children: Vector[Item]) extends Item:
    def addItem(item: Item): Unit = children = children :+ item

    override def toString: String =
        val childString = children.map(_.toString.split("\n").map("  " + _).mkString("\n")).mkString("\n")
        s"- $name (dir)\n$childString"
    
    def size: Int = children.map(_.size).sum

final case class File(name: String, size: Int) extends Item:
    override def toString: String = s"- $name (file, size=$size)"