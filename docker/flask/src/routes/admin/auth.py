from flask import Blueprint, render_template, redirect, request, url_for, flash
from werkzeug.security import check_password_hash
from flask_login import login_user,  logout_user, login_required, current_user
import os
from datetime import datetime
from string import punctuation
from ...models import User
from ...helper_functions import create_user
from werkzeug.security import generate_password_hash
from ... import config
from flask_api import status

blueprint = Blueprint('auth', __name__, url_prefix='/api/auth')

@blueprint.route('/login', methods=['POST'])
def login_post():
    """ Handles post from /login. If user and password exists in database then redirect to main.index. """
    email = request.form.get('email')
    password = request.form.get('password')
    remember = True if request.form.get('remember') else False

    user = User.query.filter_by(email=email).first()
    # check if user actually exists
    # take the user supplied password, hash it, and compare it to the hashed password in database
    if not user or not check_password_hash(user.password, password):
        return 'Please check your login details and try again.', status.HTTP_401_UNAUTHORIZED
    # if the above check passes, then we know the user has the right credentials
    login_user(user, remember=remember)
    return "Success" , status.HTTP_200_OK

@blueprint.route('/signup', methods = ['POST'])
@login_required
def signup_post():
    """ Creates new user if doesn't exist and password match requirements. Also add user to team if a team was selected. """
    if current_user.privilege < 2:
        return "Failed", status.HTTP_401_UNAUTHORIZED
    email = request.form.get('email')
    name = request.form.get('name')
    password = request.form.get('password')
    number = request.form.get('number')
    privilege = request.form.get('privilege')

    if (email == "") or (name == "") or (number == ""):
        return "All field must be filled", status.HTTP_400_BAD_REQUEST

    user = User.query.filter_by(email=email).first() # if this returns a user, then the email already exists in database

    if user: # if a user is found, we want to redirect back to signup page so user can try again
        return 'Email address already used', status.HTTP_400_BAD_REQUEST

    # Password validation
    val = True
    if len(password) < 7:
        val = False

    if not any(char.isdigit() for char in password):
        val = False

    if not any(char.isupper() for char in password):
        val = False

    if not any(char.islower() for char in password):
        val = False

    if not any(char in set(punctuation) for char in password):
        val = False

    if val == False: # if a user is found, we want to redirect back to signup page so user can try again
        return 'Password must have be atleast 8 character, including uppercase, lowercase, digit and special character', status.HTTP_400_BAD_REQUEST

    create_user(email,name,password,number,privilege)

    user = User.query.filter_by(email=email).first()

    return "Success", status.HTTP_201_CREATED

@blueprint.route("/change_password", methods = ["POST"])
@login_required
def change_password_post():
    """ Handle post to /create_password, if old password match to stored one and password satisfies requirements, then password is hashed and replaced. """
    old = request.form.get("old")
    new1 = request.form.get("new1")
    new2 = request.form.get("new2")
    if not check_password_hash(current_user.password, old):
        return 'Password is incorrect', status.HTTP_401_UNAUTHORIZED
    else:
        if new1 != new2:
            return "New passwords didn't match", status.HTTP_400_BAD_REQUEST
        else:
            # Password validation
            val = True
            if len(new2) < 7:
                val = False

            if not any(char.isdigit() for char in new2):
                val = False

            if not any(char.isupper() for char in new2):
                val = False

            if not any(char.islower() for char in new2):
                val = False

            if not any(char in set(punctuation) for char in new2):
                val = False

            if val == False: # if a user is found, we want to redirect back to signup page so user can try again
                return 'Password must have be atleast 8 character, including uppercase, lowercase, digit and special character', status.HTTP_400_BAD_REQUEST

            create_user.password = generate_password_hash(new2, method='sha256')
            return "Success", status.HTTP_200_OK


@blueprint.route('/logout')
@login_required
def logout():
    """ Logout current user. """
    logout_user()
    return "Success", status.HTTP_200_OK
