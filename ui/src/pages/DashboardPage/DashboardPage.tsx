import { useAuth0 } from '@auth0/auth0-react'
import React from 'react'
import { getUserDashboard } from '../../services/message.service'
import PageLayout from '../../components/PageLayout'

function DashboardPage() {
    const [message, setMessage] = React.useState<string>('')
    const { getAccessTokenSilently } = useAuth0()

    React.useEffect(() => {
        let isMounted = true

        const getMessage = async () => {
            const accessToken = await getAccessTokenSilently()
            const { data, error } = await getUserDashboard(accessToken)

            if (!isMounted) {
                return
            }

            if (data) {
                setMessage(JSON.stringify(data, null, 2))
            }

            if (error) {
                setMessage(JSON.stringify(error, null, 2))
            }
        }

        getMessage()

        return () => {
            isMounted = false
        }
    }, [getAccessTokenSilently])

    return (
        <PageLayout>
            <div>{message}</div>
        </PageLayout>
    )
}

export default DashboardPage
