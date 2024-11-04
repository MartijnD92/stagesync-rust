import { useAuth0 } from '@auth0/auth0-react'
import PageLoader from './components/PageLoader'
import AuthenticationGuard from './components/AuthenticationGuard'
import { Route, Routes } from 'react-router-dom'
import HomePage from './pages/HomePage'
import CallbackPage from './pages/CallbackPage'
import NotFoundPage from './pages/NotFoundPage'
import ProfilePage from './pages/ProfilePage'
import ArtistOverviewPage from './pages/ArtistOverviewPage'

function App() {
    const { isLoading } = useAuth0()

    if (isLoading) {
        return (
            <div className="page-layout">
                <PageLoader />
            </div>
        )
    }

    return (
        <Routes>
            <Route path="/" element={<HomePage />} />
            <Route
                path="/artists"
                element={<AuthenticationGuard component={ArtistOverviewPage} />}
            />
            <Route
                path="/profile"
                element={<AuthenticationGuard component={ProfilePage} />}
            />
            <Route
                path="/404"
                element={<AuthenticationGuard component={NotFoundPage} />}
            />
            <Route path="/callback" element={<CallbackPage />} />
            <Route path="*" element={<NotFoundPage />} />
        </Routes>
    )
}

export default App
