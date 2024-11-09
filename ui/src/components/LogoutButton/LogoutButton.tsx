import { useAuth0 } from '@auth0/auth0-react'
import Button from '../Button'
import s from '../Button/Button.module.css'

function LogoutButton() {
    const { logout } = useAuth0()

    const handleLogout = () => {
        logout({
            logoutParams: {
                returnTo: window.location.origin,
            },
        })
    }

    return (
        <Button extraStyles={`${s.secondary}`} onClickHandler={handleLogout}>
            Log out
        </Button>
    )
}

export default LogoutButton
