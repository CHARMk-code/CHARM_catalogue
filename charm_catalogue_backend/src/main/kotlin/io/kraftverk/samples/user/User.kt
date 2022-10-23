package io.kraftverk.samples.user

import javax.persistence.Entity
import javax.persistence.GeneratedValue
import javax.persistence.Id

@Entity
class User(
    @Id
    @GeneratedValue
    var id: Long = 0,
    var email: String,
    var name: String,
    var bio: String? = null
)
