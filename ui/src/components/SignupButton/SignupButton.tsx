import { useAuth0 } from '@auth0/auth0-react'
import Button from '../Button'
import s from '../Button/Button.module.css'

function SignupButton() {
    const { loginWithRedirect } = useAuth0()

    const handleSignUp = async () => {
        await loginWithRedirect({
            appState: {
                returnTo: '/profile',
            },
            authorizationParams: {
                prompt: 'login',
                screen_hint: 'signup',
            },
        })
    }

    return (
        <Button extraStyles={`${s.primary}`} onClickHandler={handleSignUp}>
            Sign up
        </Button>
    )
}

export default SignupButton
