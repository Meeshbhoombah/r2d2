from app import app
from flask import render_template, flash, redirect 
from app.forms import LoginForm, SigninForm


@app.route('/')
@app.route('/index')
def index():
	
	return render_template('index.html', title='Home')


@app.route('/login', methods=['GET','POST'])
def login():
	form = LoginForm()
	if form.validate_on_submit():
		flash('Login requested for user {}'.format(form.username.data))
		return redirect(url_for(index))
	return render_template('login.html', title="log in", form=form)


@app.route('/join', methods=['GET','POST'])
def join():
	return render_template('join.html', title="join")

@app.route('/settings' methods=['GET'])
def settings():
	return render_template('settings.html', title="settings")