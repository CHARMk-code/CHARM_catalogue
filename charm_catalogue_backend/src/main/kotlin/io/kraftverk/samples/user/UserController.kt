package io.kraftverk.samples.user

import io.javalin.Javalin
import io.javalin.apibuilder.ApiBuilder.get
import io.javalin.apibuilder.ApiBuilder.path
import io.javalin.apibuilder.ApiBuilder.post
import io.javalin.apibuilder.ApiBuilder.put
import io.javalin.http.Context
import io.javalin.plugin.openapi.annotations.OpenApi
import io.javalin.plugin.openapi.annotations.OpenApiContent
import io.javalin.plugin.openapi.annotations.OpenApiRequestBody
import io.javalin.plugin.openapi.annotations.OpenApiResponse

class UserController(private val userService: UserService) {

    @OpenApi(
        summary = "Create user",
        tags = ["User"],
        requestBody = OpenApiRequestBody(content = [OpenApiContent(User::class)]),
        responses = [OpenApiResponse("200", [OpenApiContent(User::class)])]
    )
    fun create(ctx: Context) {
        val user = ctx.body<User>()
        userService.create(user)
        ctx.json(user)
    }

    @OpenApi(
        summary = "Read user",
        tags = ["User"],
        responses = [
            OpenApiResponse("200", [OpenApiContent(User::class)]),
            OpenApiResponse("404")
        ]
    )
    fun read(ctx: Context) {
        val id = ctx.pathParam("userId").toLong()
        val user = userService.findById(id)
        if (user != null) ctx.json(user) else ctx.status(404)
    }

    @OpenApi(
        summary = "Update user",
        tags = ["User"],
        requestBody = OpenApiRequestBody(content = [OpenApiContent(User::class)]),
        responses = [OpenApiResponse("204")]
    )
    fun update(ctx: Context) {
        val user = ctx.body<User>()
        userService.update(user)
        ctx.status(204)
    }
}

internal fun Javalin.expose(userController: UserController) = routes {
    path("users") {
        post(userController::create)
        path(":userId") {
            get(userController::read)
        }
        put(userController::update)
    }
}
