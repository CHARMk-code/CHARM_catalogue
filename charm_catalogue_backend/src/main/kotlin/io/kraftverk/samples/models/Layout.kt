

import javax.persistence.Entity
import javax.persistence.GeneratedValue
import javax.persistence.Id

@Entity
class Prepage(
    @Id
    @GeneratedValue
    var id: Long = 0,
    var image: String,
    var placement: int, //Borde göra enum för detta
    var active: Boolean
)
