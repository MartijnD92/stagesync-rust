import { useAuth0 } from '@auth0/auth0-react'
import Button from '../Button'
import s from '../Button/Button.module.css'

function LoginButton() {
    const { loginWithRedirect } = useAuth0()

    const handleLogin = async () => {
        await loginWithRedirect({
            appState: {
                returnTo: '/dashboard',
            },
            authorizationParams: {
                prompt: 'login',
            },
        })
    }

    return (
        <Button extraStyles={`${s.secondary}`} onClickHandler={handleLogin}>
            Log in
        </Button>
    )
}

export default LoginButton
