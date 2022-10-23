import javax.persistence.Entity
var description: String,
var link:string,
var icon:String
import javax.persistence.GeneratedValue
import javax.persistence.Id

@Entity
class Tag(
    @Id
    @GeneratedValue
    var id: Long = 0,
    var name: String,
    var icon: String,
    var division: Boolean,
    var looking_for: Boolean,
    var offering: Boolean,
    var language: Boolean
)
