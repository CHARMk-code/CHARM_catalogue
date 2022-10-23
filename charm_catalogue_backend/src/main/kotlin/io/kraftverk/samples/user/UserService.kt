package io.kraftverk.samples.user

import io.kraftverk.samples.hibernate.Tx
import io.kraftverk.samples.hibernate.read
import io.kraftverk.samples.hibernate.write

class UserService(
    private val tx: Tx,
    private val repository: UserRepository
) {

    fun findById(id: Long): User? = tx.read { repository.findById(id) }

    fun create(user: User) {
        doWrite(user, repository::create)
    }

    fun update(user: User) {
        doWrite(user, repository::update)
    }

    private fun doWrite(user: User, write: (User) -> Unit) {
        tx.write {
            require(!repository.emailExists(user)) { "Email already exists" }
            require(!repository.nameExists(user)) { "Name already exists" }
            write(user)
        }
    }
}
