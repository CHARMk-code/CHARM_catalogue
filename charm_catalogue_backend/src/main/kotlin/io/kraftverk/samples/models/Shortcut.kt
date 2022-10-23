
import javax.persistence.Entity
import javax.persistence.GeneratedValue
import javax.persistence.Id

@Entity
class Shortcut(
    @Id
    @GeneratedValue
    var id: Long = 0,
    var name: String,
    var description: String,
    var link:string,
    var icon:String
)
