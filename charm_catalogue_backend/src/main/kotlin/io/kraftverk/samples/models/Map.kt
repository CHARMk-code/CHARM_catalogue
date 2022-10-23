import javax.persistence.Entity
import javax.persistence.GeneratedValue
import javax.persistence.Id

@Entity
class Map(
    @Id
    @GeneratedValue
    var id: Long = 0,
    var name: String,
    var image: String,
    @ManyToOne
    var ref: Map
)
