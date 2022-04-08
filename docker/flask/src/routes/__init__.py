import sys, os
from os.path import dirname, basename, isfile, join
import glob
from importlib import import_module

def register_all_blueprints(app):
    """ Registers all route with the app"""
    basedir = "./routes"
    modules = [y for x in os.walk(basedir) for y in glob.glob(os.path.join(x[0], '*.py'))]
    routes = []
    for f in modules:
        if f.endswith('__init__.py') or not isfile(f):
            continue

        routes.append(dirname(f)[len(basedir)+1:].replace('/','.') + "." +basename(f)[:-3])

    for route in routes:
        if route[0] == '.':
            route = route[1:]

        route_module = import_module('catalogue.routes.' + route)
        app.register_blueprint(route_module.blueprint)
