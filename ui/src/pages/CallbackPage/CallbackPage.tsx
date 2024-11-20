import { useAuth0 } from '@auth0/auth0-react'
// import { NavBar } from '../components/navigation/desktop/nav-bar'
// import { MobileNavBar } from '../components/navigation/mobile/mobile-nav-bar'
import PageLayout from '../../components/PageLayout'

function CallbackPage() {
    const { error } = useAuth0()

    if (error) {
        return (
            <PageLayout>
                <div className="content-layout">
                    <h1 id="page-title" className="content__title">
                        Error
                    </h1>
                    <div className="content__body">
                        <p id="page-description">
                            <span>{error.message}</span>
                        </p>
                    </div>
                </div>
            </PageLayout>
        )
    }

    return <></>
}

export default CallbackPage
