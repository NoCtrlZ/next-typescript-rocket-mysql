import React, { Component } from 'react'
import Link from 'next/link'
import Axios from 'axios'
import { NextPage } from 'next'
import '../styles/login.scss'

class LoginPage extends React.Component<{}, { user_mail: string, user_password: string}> {
    constructor(props: any) {
        super(props);
        this.state = { user_mail: '', user_password: ''}

        this.emailChange = this.emailChange.bind(this)
        this.passwordChange = this.passwordChange.bind(this)
        this.sendTemporaryUser = this.sendTemporaryUser.bind(this)
    }
    emailChange(event: any) {
        this.setState({user_mail: event.target.value})
    }
    passwordChange(event: any) {
        this.setState({user_password: event.target.value})
    }
    sendTemporaryUser() {
        Axios.get("http://localhost:8000")
        .then(function (response) {
            console.log(response)
        })
    }
    render() {
        return(
            <div id="signin">
                <Link href="/">
                <a>Home</a>
                </Link>
                <Link href="/about">
                <a>About</a>
                </Link>
                <h1>This Is Signin Page</h1>
                <p>Please Fill In Your Registeration Information</p>
                <div id="register-form">
                    <p>Mail</p>
                    <p><input name="user_mail" type="text" value={this.state.user_mail} onChange={this.emailChange}/></p>
                    <p>Password</p>
                    <p><input name="user_password" type="text" value={this.state.user_password} onChange={this.passwordChange}/></p>
                    <p><button onClick={ this.sendTemporaryUser }>Send</button></p>
                </div>
                <h1>{ this.state.user_mail }</h1>
                <h1>{ this.state.user_password }</h1>
            </div>
        )
    }
    registerUser(e: any) {
        this.setState({ user_mail: e, user_password: e})
    }
}

export default LoginPage
