import { useAuth0 } from '@auth0/auth0-react'
import PageLayout from '../../components/PageLayout'
import LoginButton from '../../components/LoginButton'
import LogoutButton from '../../components/LogoutButton'
import SignupButton from '../../components/SignupButton'
import s from './HomePage.module.css'

function HomePage() {
    const { isAuthenticated } = useAuth0()
    return (
        <PageLayout>
            <>
                <h1>Welcome to StageSync</h1>
                <div className={s['navbar-buttons']}>
                    {!isAuthenticated && (
                        <>
                            <SignupButton />
                            <LoginButton />
                        </>
                    )}
                    {isAuthenticated && (
                        <>
                            <LogoutButton />
                        </>
                    )}
                </div>
            </>
        </PageLayout>
    )
}

export default HomePage
