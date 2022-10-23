package io.kraftverk.samples.user

import org.hibernate.Session
import org.hibernate.SessionFactory

open class UserRepository(private val sessionFactory: SessionFactory) {

    private val session get() = sessionFactory.currentSession

    fun findById(id: Long): User? = session.get(User::class.java, id)

    fun emailExists(user: User): Boolean = session
        .createExistsQuery("u.email = :email and u.id <> :id")
        .setParameter("email", user.email)
        .setParameter("id", user.id)
        .uniqueResult() != null

    fun nameExists(user: User): Boolean = session
        .createExistsQuery("u.name = :name and u.id <> :id")
        .setParameter("name", user.name)
        .setParameter("id", user.id)
        .uniqueResult() != null

    fun create(user: User) {
        session.save(user)
    }

    fun update(user: User) {
        session.update(user)
    }

    private fun Session.createExistsQuery(condition: String) =
        createQuery("select 1 from User where exists (select 1 from User u where $condition)")
}
