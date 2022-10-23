
import javax.persistence.Entity
import javax.persistence.GeneratedValue
import javax.persistence.Id

@Entity
class Prepage(
    @Id
    @GeneratedValue
    var id: Long = 0,
    var name: String,
    var order: int,
    var active: Boolean
)
