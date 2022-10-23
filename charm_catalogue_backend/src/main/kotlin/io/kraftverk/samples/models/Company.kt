import javax.persistence.Entity
import javax.persistence.GeneratedValue
import javax.persistence.Id

@Entity
class Company(
    @Id
    @GeneratedValue
    var id: Long = 0,
    var name: String ,
    var description: String,
    var last_updated: datetime = datetime.now()
    var contacts: String,
    var contact_email: String,
    var employees_world: int,
    var website: String,
    var talk_to_us_about: String,
    var logo: String,
    var map_image: String,
    var booth_number: int,
    var charmtalk: Boolean,
    var summer_job: Boolean,
    var summer_job_description: String,
    var summer_job_link: String,

    @OneToMany
    var tags: Tags
)
