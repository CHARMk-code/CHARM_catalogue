package io.kraftverk.samples

import io.kraftverk.core.Kraftverk
import io.kraftverk.core.module.Module
import io.kraftverk.core.module.module
import io.kraftverk.samples.hibernate.HibernateModule
import io.kraftverk.samples.hikari.HikariModule
import io.kraftverk.samples.javalin.JavalinModule
import io.kraftverk.samples.user.User
import io.kraftverk.samples.user.UserController
import io.kraftverk.samples.user.UserRepository
import io.kraftverk.samples.user.UserService
import io.kraftverk.samples.user.expose
import org.hibernate.cfg.Environment.DATASOURCE

fun main() {
    Kraftverk.start { AppModule() }
}

class AppModule : Module() {

    val jdbc by module { HikariModule() }
    val orm by module { HibernateModule() }
    val http by module { JavalinModule() }

    val userRepository by bean { UserRepository(orm.sessionFactory()) }
    val userService by bean { UserService(orm.tx(), userRepository()) }
    val userController by bean { UserController(userService()) }

    init {
        configure(orm.serviceRegistryBuilder) {
            it.applySetting(DATASOURCE, jdbc.dataSource())
        }
        configure(orm.metadataSources) {
            it.addAnnotatedClass(User::class.java)
        }
        configure(http.javalin) {
            it.expose(userController())
        }
    }
}
